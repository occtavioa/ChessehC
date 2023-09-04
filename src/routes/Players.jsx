import { invoke } from "@tauri-apps/api";
import { useEffect, useRef } from "react"
import { useLoaderData, useParams } from "react-router"

function Players() {
    const {path} = useParams()
    const players = useLoaderData()
    const formDialogRef = useRef(null);

    useEffect(() => {
        console.log(players);
    }, [])

    return (
        <>
            Jugadores

            <button onClick={() => {
                formDialogRef.current.showModal()
            }}>Agregar jugador</button>

            <dialog ref={formDialogRef}>
                <button onClick={() => {
                    formDialogRef.current.close()
                }}>x</button>
                
                <form onSubmit={async (e) => {
                    e.preventDefault()

                    let player = Object.fromEntries(new FormData(e.target))
                    player.rating = parseInt(player.rating)
                    player.fide_number = parseInt(player.fide_number)

                    console.log(player);
                    invoke("create_player", {path: atob(path), player: player})
                        .then((player) => {console.log(player);})
                        .catch((error) => {console.error(error);})
                        .finally(() => {formDialogRef.current.close()})
                }}>
                    <label htmlFor="name">Nombre</label>
                    <input type="text" name="name" id="name" required />

                    <label htmlFor="sex">Sexo</label>
                    <input type="text" name="sex" id="sex" />

                    <label htmlFor="title">Titulo</label>
                    <input type="text" name="title" id="title" />

                    <label htmlFor="rating">Rating</label>
                    <input type="number" name="rating" id="rating" min={0} max={9999} defaultValue={0} />

                    <label htmlFor="fideFederation">Federacion</label>
                    <input type="number" name="fide_federation" id="fideFederation" />

                    <label htmlFor="fideNumber">Número fide</label>
                    <input type="number" name="fide_number" id="fideNumber" />

                    <label htmlFor="birthDate">Fecah de nacimiento</label>
                    <input type="date" name="birth_date" id="birthDate" />

                    <button type="submit">Agregar</button>
                </form>
            </dialog>
        </>
    )
}

export default Players
