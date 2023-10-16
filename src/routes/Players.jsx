import { invoke } from "@tauri-apps/api";
import { useEffect, useRef, useState } from "react"
import { useHref, useNavigate, useParams } from "react-router"
import { Form, useFetcher, useLoaderData } from "react-router-dom";

function Players() {
    const fetcher = useFetcher()
    const href = useHref()
    const formDialogRef = useRef(null);
    const [players, setPlayers] = useState([])

    useEffect(() => {
        if(fetcher.state === "idle" && !fetcher.data) {
            fetcher.load(href)
        }
    }, [fetcher.state])

    useEffect(() => {
        if(fetcher.data) {
            setPlayers(fetcher.data)
        }
    }, [fetcher.data])

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
                        <th>Rating</th>
                        <th>Título</th>
                        <th>Nombre</th>
                        <th>Puntos</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        players.map((p, i) => 
                            <tr key={i}>
                                <td>{p.id}</td>
                                <td>{p.rating}</td>
                                <td>{p.title}</td>
                                <td>{p.name}</td>
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

                <fetcher.Form method="post" onSubmit={() => {
                    formDialogRef.current.close()
                }}>
                    <label htmlFor="name">Nombre</label>
                    <input type="text" name="name" id="name" required />

                    <label htmlFor="title">Título</label>
                    <select name="title" id="title">
                        <option value="">Ninguno</option>
                        <option>WCM</option>
                        <option>WFM</option>
                        <option>CM</option>
                        <option>WIM</option>
                        <option>FM</option>
                        <option>WGM</option>
                        <option>IM</option>
                        <option>GM</option>
                    </select>

                    <label htmlFor="rating">Rating</label>
                    <input type="number" name="rating" id="rating" min={0} max={9999} defaultValue={0} />

                    <button type="submit">Agregar</button>
                </fetcher.Form>
            </dialog>
        </>
    )
}

export default Players
