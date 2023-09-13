import { invoke } from "@tauri-apps/api";
import { useEffect, useRef } from "react"
import { useNavigate, useParams } from "react-router"

function Players() {
    const {path} = useParams()
    const formDialogRef = useRef(null);
    const playersTableRef = useRef(null);
    const navigate = useNavigate()

    useEffect(() => {
        invoke("get_players", {path: atob(path)})
            .then((players) => {
                console.log(players);
            })
            .catch((error) => {
                console.error(error);
                navigate("/error")
            })
    }, [])
    
    return (
        <>
            Jugadores

            <button onClick={() => {
                formDialogRef.current.showModal()
            }}>Agregar jugador</button>

            <table>
                <thead>
                    <tr>
                        <th>Nombre</th>
                        <th>Título</th>
                        <th>Rating</th>
                        <th>Federación</th>
                        <th>Número FIDE</th>
                        <th>Fecha de nacimiento</th>
                    </tr>
                </thead>
                <tbody ref={playersTableRef}>
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
                    player.rating = parseInt(player.rating)
                    player.points = 0.0;

                    console.log(player);
                    invoke("create_player", {path: atob(path), player: player})
                        .then((player) => {console.log(player);})
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
