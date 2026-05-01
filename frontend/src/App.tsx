import { useState, useEffect } from "react";
import { MapContainer, TileLayer, Marker, Popup, useMap } from "react-leaflet";
import L from "leaflet";
import "leaflet/dist/leaflet.css";
import "./App.css";

// Correção para ícones do Leaflet no React
import markerIcon from "leaflet/dist/images/marker-icon.png";
import markerShadow from "leaflet/dist/images/marker-shadow.png";

let DefaultIcon = L.icon({
  iconUrl: markerIcon,
  shadowUrl: markerShadow,
  iconSize: [25, 41],
  iconAnchor: [12, 41],
});

L.Marker.prototype.options.icon = DefaultIcon;

interface Interdicao {
  motivo: string;
  status: string;
}

interface Pmqc {
  parametro: string;
  conforme: boolean;
}

interface PostoCompleto {
  cnpj: string;
  razao_social: string;
  endereco: string;
  municipio: string;
  status_autorizacao: string | null;
  interdicoes: Interdicao[];
  pmqc: Pmqc[];
  latitude?: number;
  longitude?: number;
}

function ChangeView({ center }: { center: [number, number] }) {
  const map = useMap();
  map.flyTo(center, 15, { duration: 1.5 });
  return null;
}

function App() {
  const [search, setSearch] = useState("");
  const [stations, setStations] = useState<PostoCompleto[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedPosition, setSelectedPosition] = useState<[number, number] | null>(null);
  const [selectedCnpj, setSelectedCnpj] = useState<string | null>(null);

  const getMarkerIcon = (station: PostoCompleto) => {
    const isInterditado = station.interdicoes.some(i => i.status === "INTERDITADO");
    const hasQualidadeRuim = station.pmqc.some(p => !p.conforme);
    
    let colorClass = "marker-green";
    if (isInterditado || hasQualidadeRuim) colorClass = "marker-red";
    if (station.status_autorizacao !== "ATIVO") colorClass = "marker-grey";

    return L.divIcon({
      className: `custom-marker ${colorClass}`,
      iconSize: [14, 14],
      iconAnchor: [7, 7],
    });
  };

  useEffect(() => {
    const delayDebounceFn = setTimeout(() => {
      setLoading(true);
      setError(null);
      
      const queryParam = search.trim() ? `?q=${encodeURIComponent(search)}` : '';
      
      fetch(`http://localhost:3000/api/postos/search${queryParam}`)
        .then((res) => {
          if (!res.ok) throw new Error("Erro na rede ao buscar postos");
          return res.json();
        })
        .then((data) => {
          setStations(data);
          setLoading(false);
        })
        .catch((err) => {
          console.error(err);
          setError("Não foi possível conectar à API. Verifique se o 'cargo run --bin api' está rodando.");
          setLoading(false);
        });
    }, 500);

    return () => clearTimeout(delayDebounceFn);
  }, [search]);

  return (
    <div className="app-container">
      <aside className="sidebar">
        <h1>⛽ ParâmetroPostos</h1>
        
        <input 
          type="text" 
          className="search-box" 
          placeholder="Buscar por CNPJ, Nome ou Cidade..." 
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />

        <div className="status-list">
          <p style={{ fontSize: 12, color: 'var(--text-secondary)', marginTop: 12 }}>
            Resultados {loading && "(Carregando...)"}
          </p>
          
          {error && <p style={{ fontSize: 12, color: 'var(--accent-red)' }}>{error}</p>}
          
          {stations.map(station => {
            const isInterditado = station.interdicoes.some(i => i.status === "INTERDITADO");
            const hasQualidadeRuim = station.pmqc.some(p => !p.conforme);
            const hasNoInfo = station.interdicoes.length === 0 && station.pmqc.length === 0;

            return (
              <div 
                key={station.cnpj} 
                className={`station-card ${selectedCnpj === station.cnpj ? 'active' : ''}`}
                onClick={() => {
                  setSelectedCnpj(station.cnpj);
                  if (station.latitude && station.longitude) {
                    setSelectedPosition([station.latitude, station.longitude]);
                  }
                }}
              >
                <h3>{station.razao_social}</h3>
                <p>{station.endereco} - {station.municipio}</p>
                <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap' }}>
                  <span className={`badge ${station.status_autorizacao === 'ATIVO' && !isInterditado ? 'success' : 'danger'}`}>
                    {isInterditado ? 'Interditado ANP' : station.status_autorizacao || 'Desconhecido'}
                  </span>
                  {!hasNoInfo && station.pmqc.length > 0 && (
                    <span className={`badge ${hasQualidadeRuim ? 'danger' : 'success'}`}>
                      {hasQualidadeRuim ? 'PMQC Reprovado' : 'PMQC Aprovado'}
                    </span>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      </aside>

      <main className="main-content">
        <MapContainer center={[-15.7801, -47.9292]} zoom={4} style={{ height: "100%", width: "100%" }}>
          <TileLayer
            url="https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png"
            attribution='&copy; OpenStreetMap &copy; CARTO'
          />
          {selectedPosition && <ChangeView center={selectedPosition} />}
          {stations.filter(s => s.latitude && s.longitude).map(s => (
            <Marker 
              key={s.cnpj} 
              position={[s.latitude!, s.longitude!]} 
              icon={getMarkerIcon(s)}
              eventHandlers={{
                click: () => setSelectedCnpj(s.cnpj),
              }}
            >
              <Popup>
                <div className="popup-container">
                  <h4>{s.razao_social}</h4>
                  <p><strong>CNPJ:</strong> {s.cnpj}</p>
                  <p>{s.endereco}</p>
                  <div style={{ display: 'flex', gap: 4, marginTop: 4 }}>
                    <span className={`badge ${s.status_autorizacao === 'ATIVO' ? 'success' : 'danger'}`}>
                      {s.interdicoes.some(i => i.status === "INTERDITADO") ? '⚠️ Interditado' : s.status_autorizacao}
                    </span>
                    {s.pmqc.length > 0 && (
                      <span className={`badge ${s.pmqc.some(p => !p.conforme) ? 'danger' : 'success'}`}>
                        {s.pmqc.some(p => !p.conforme) ? '🚫 Qualidade' : '✅ Qualidade'}
                      </span>
                    )}
                  </div>
                </div>
              </Popup>
            </Marker>
          ))}
        </MapContainer>
      </main>
    </div>
  );
}

export default App;
