import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { useParams } from "react-router";

function Standings() {
    const {path, roundId} = useParams()
    const [standings, setStandings] = useState([])
    
    useEffect(() => {
        invoke("get_standings_by_round", {path: atob(path), roundId: parseInt(roundId)})
            .then((standings) => {setStandings(standings)})
            .catch((error) => {console.error(error);})
    }, [path, roundId])
    
    return (
        <>
            Clasificación ronda {}
            <table>
                <thead>
                    <tr>
                        <th>Id</th>
                        <th>Nombre</th>
                        <th>Rating</th>
                        <th>Puntos</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        standings.map((p, i) => 
                            <tr key={i}>
                                <td>{p.id}</td>
                                <td>{p.name}</td>
                                <td>{p.rating}</td>
                                <td>{p.points}</td>
                            </tr>
                        )
                    }
                </tbody>
            </table>
        </>
    )
}

export default Standings
