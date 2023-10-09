import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { Link, Outlet, useNavigate, useParams } from "react-router-dom"

function TournamentLayout() {
    const {path} = useParams()
    const [rounds, setRounds] = useState([])
    const [selectedRoundId, setSelectedRoundId] = useState()
    const navigate = useNavigate()

    useEffect(() => {
        invoke("get_rounds", {path: atob(path)})
            .then((rounds) => {
                setRounds(rounds)
                setSelectedRoundId(rounds.at(-1).id)
                console.log(rounds.at(-1));
            })
            .catch(e => {
                console.error(e);
            })
    }, [path])

    useEffect(() => {
        console.log(selectedRoundId);
    }, [selectedRoundId])
    
    return (
        <>
            <nav>
                <Link to={`/`}>Inicio</Link>
                <Link to={`.`}>Torneo</Link>
                <Link to={`players`}>Jugadores</Link>
                <select value={selectedRoundId} onChange={(e) => {setSelectedRoundId(e.target.value)}}>
                    {
                        rounds.map(r => {
                            return <option value={r.id} key={r.id}>Ronda {r.number}</option>
                        })
                    }
                </select>
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
