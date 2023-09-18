import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { Link, Outlet, useNavigate, useParams } from "react-router-dom"

function TournamentLayout() {
    const {path} = useParams()
    const [currentRound, setCurrentRound] = useState()
    const [selectedRound, setSelectedRound] = useState();
    const navigate = useNavigate()

    useEffect(() => {
        invoke("get_current_round", {path: atob(path)})
            .then((round) => {
                if (Number.isInteger(round) && round > 0) {
                    setCurrentRound(round)
                    setSelectedRound(round)
                }
            })
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
                    Number.isInteger(currentRound) && currentRound > 0 ?
                        <>
                            <select name="" id="" value={selectedRound} onChange={(e) => {setSelectedRound(parseInt(e.target.value))}}>
                                {[...Array(currentRound)].map((_n, i) =>
                                    <option key={i} value={i+1}>Ronda {i+1}</option>
                                )}
                            </select>
                            <Link to={`${selectedRound}/pairings`}>Pareos</Link>
                            <Link to={`${selectedRound}/standings`}>Posiciones</Link>
                        </> :
                        <></>
                }
                <button onClick={async () => {
                    invoke("make_pairing", {path: atob(path)})
                        .then((nextRound) => {
                            setCurrentRound(nextRound)
                            if(nextRound === 1) {
                                setSelectedRound(nextRound)
                            }
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
