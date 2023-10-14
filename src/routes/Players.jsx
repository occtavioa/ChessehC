import { invoke } from "@tauri-apps/api";
import { useEffect, useRef, useState } from "react"
import { useNavigate, useParams } from "react-router"
import { useLoaderData } from "react-router-dom";

function Players() {
    const {path} = useParams()
    const formDialogRef = useRef(null);
    const [players, setPlayers] = useState(useLoaderData())
    const navigate = useNavigate()
    
    return (
        <>
            Jugadores

            <button onClick={() => {
                formDialogRef.current.showModal()
            }}>Agregar jugador</button>

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
                        players.map((p, i) => 
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

            <dialog ref={formDialogRef}>
                <button onClick={() => {
                    formDialogRef.current.close()
                }}>x</button>
                
                <form onSubmit={async (e) => {
                    e.preventDefault()

                    let player = Object.fromEntries(new FormData(e.target))
                    player.id = 0
                    player.tournament_id = 0
                    player.points = 0.0
                    player.rating = parseInt(player.rating)

                    invoke("add_player", {path: atob(path), player: player})
                        .then(() => {
                            invoke("get_players", {path: atob(path)})
                                .then((players) => {
                                    setPlayers(players)
                                })
                                .catch((error) => {
                                    console.error(error);
                                    navigate("/error")
                                })
                        })
                        .catch((error) => {console.error(error);})
                        .finally(() => {formDialogRef.current.close()})
                }}>
                    <label htmlFor="name">Nombre</label>
                    <input type="text" name="name" id="name" required />

                    <label htmlFor="rating">Rating</label>
                    <input type="number" name="rating" id="rating" min={0} max={9999} defaultValue={0} />

                    <button type="submit">Agregar</button>
                </form>
            </dialog>
        </>
    )
}

export default Players
