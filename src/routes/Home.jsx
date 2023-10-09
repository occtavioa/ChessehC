import { invoke } from "@tauri-apps/api";
import { useRef } from "react";
import { useNavigate} from "react-router-dom";

function Home() {
    const formDialogRef = useRef(null);
    const navigate = useNavigate()

    return (
        <>
            <img src="" alt="chessehc-logo" />

            <button onClick={() => {
                formDialogRef.current.showModal();
            }}>Nuevo torneo</button>

            <button onClick={async () => {
                invoke("pick_tournament_file")
                    .then((path) => {if(path) navigate(`tournament/${btoa(path)}`)})
                    .catch((error) => {console.error(error)})
            }}>Cargar torneo</button>

            <dialog ref={formDialogRef}>
                <button onClick={() => {
                    formDialogRef.current.close();
                }}>x</button>

                <form onSubmit={(e) => {
                    e.preventDefault();

                    let tournament = Object.fromEntries(new FormData(e.target));
                    tournament.id = 0;
                    tournament.number_rounds = parseInt(tournament.number_rounds);
                    tournament.current_round_id = null;

                    invoke("create_tournament", {tournament: tournament})
                        .then((path) => {
                            if(path) {
                                console.log("Torneo creado en", path);
                            }
                        })
                        .catch((error) => {console.error(error);})
                        .finally(() => {formDialogRef.current.close()})
                }}>
                    <label htmlFor="name">Nombre</label>
                    <input type="text" name="name" id="name" required/>

                    <label htmlFor="numberRounds">Número de rondas</label>
                    <input type="number" name="number_rounds" id="numberRounds" required min={5} max={9999}/>

                    <button type="submit">Crear</button>
                </form>
            </dialog>
        </>
    )
}

export default Home;
