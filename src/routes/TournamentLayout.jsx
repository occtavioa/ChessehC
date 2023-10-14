import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { Link, Outlet, useHref, useLocation, useNavigate, useParams, useResolvedPath, useSearchParams } from "react-router-dom"

function TournamentLayout() {
    const {path, roundId} = useParams()
    const [rounds, setRounds] = useState([])
    const [selectedRoundId, setSelectedRoundId] = useState()
    const {pathname} = useResolvedPath()
    const navigate = useNavigate()
    
    useEffect(() => {
        invoke("get_rounds", {path: atob(path)})
            .then((rounds) => {
                if(rounds.length > 0) {
                    setRounds(rounds)
                    setSelectedRoundId(rounds.at(-1).id)
                }
            })
            .catch(e => {
                console.error(e);
            })
    }, [path])
    
    useEffect(() => {
        if(roundId && selectedRoundId) {
            navigate(`round/${selectedRoundId}/${pathname.split("/").at(-1)}`)
        }
    }, [selectedRoundId])
    
    return (
        <>
            <nav>
                <Link to={`/`}>Inicio</Link>
                <Link to={`.`}>Torneo</Link>
                <Link to={`players`}>Jugadores</Link>
                {
                    rounds.length > 0 &&
                        <select value={selectedRoundId} onChange={(e) => {setSelectedRoundId(parseInt(e.target.value))}}>
                            {
                                rounds.map(r => 
                                    <option value={r.id} key={r.id}>Ronda {r.number}</option>
                                )
                            }
                        </select>
                }
                {
                    selectedRoundId &&
                        <>
                            <Link to={`round/${selectedRoundId}/pairings`}>Pareos</Link>
                            <Link to={`round/${selectedRoundId}/standings`}>Clasificación</Link>
                        </>
                }
                <button onClick={async () => {
                    invoke("make_pairing", {path: atob(path)})
                        .then(() => {
                            invoke("get_rounds", {path: atob(path)})
                                .then((rounds) => {
                                    setRounds(rounds)
                                    if(!selectedRoundId) {
                                        setSelectedRoundId(rounds.at(-1).id)
                                    }
                                })
                                .catch(e => {
                                    console.error(e);
                                })
                        })
                        .catch((error) => {
                            console.error(error);
                        })
                }}>Realizar pareo</button>
            </nav>
            <Outlet></Outlet>
        </>
    )
}

export default TournamentLayout;
