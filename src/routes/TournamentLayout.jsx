import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { Link, Outlet, useNavigate, useParams } from "react-router-dom"

function TournamentLayout() {
    const {path} = useParams()
    const [selectedRound, setSelectedRound] = useState();
    const navigate = useNavigate()

    useEffect(() => {
        invoke("get_current_round", {path: atob(path)})
            .then((round) => {setSelectedRound(round)})
            .catch((error) => {
                console.error(error);
                navigate("/error")
            })
    }, [])
    
    return (
        <>
            <nav>
                <Link to={`/`}>Inicio</Link>
                <Link to={`.`}>Torneo</Link>
                <Link to={`players`}>Jugadores</Link>
                {
                    Number.isInteger(selectedRound) ?
                        <select name="" id="" value={selectedRound} onChange={(e) => {setSelectedRound(e.target.value)}}>
                            {[...Array(selectedRound)].map((_n, i) => 
                                <option key={i} value={i+1}>Ronda {i+1}</option>
                            )}
                        </select> :
                        <></>
                }
                <button onClick={async () => {
                    invoke("make_pairing", {path: atob(path)})
                        .then((nextRound) => {setSelectedRound(nextRound)})
                        .catch((error) => {
                            console.error(error);
                            navigate("/error")
                        })
                }}>Realizar pareo</button>
            </nav>
            <Outlet></Outlet>
        </>
    )
}

export default TournamentLayout;
