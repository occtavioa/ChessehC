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
                    tournament.number_rounds = parseInt(tournament.number_rounds);
                    tournament.current_round = 0;

                    invoke("create_tournament", {tournament: tournament})
                        .then((path) => {console.log("Torneo creado en", path);})
                        .catch((error) => {console.error(error);})
                        .finally(() => {formDialogRef.current.close()})
                }}>
                    <label htmlFor="name">Nombre</label>
                    <input type="text" name="name" id="name" required/>

                    <label htmlFor="city">Ciudad</label>
                    <input type="text" name="city" id="city" />

                    <label htmlFor="fideFederation">Federación</label>
                    <input type="text" name="fide_federation" id="fideFederation" />

                    <label htmlFor="dateStart">Inicio</label>
                    <input type="date" name="date_start" id="dateStart" />

                    <label htmlFor="dateEnd">Fin</label>
                    <input type="date" name="date_end" id="dateEnd" />

                    <label htmlFor="typeTournament">Tipo de torneo</label>
                    <input type="text" name="type_tournament" id="typeTournament" />

                    <label htmlFor="format">Formato</label>
                    <select name="format" id="format">
                        <option value="Swiss">Suizo</option>
                    </select>

                    <label htmlFor="chiefArbiter">Árbitro principal</label>
                    <input type="text" name="chief_arbiter" id="chiefArbiter" />

                    <label htmlFor="chiefArbiter">Árbitro suplente</label>
                    <input type="text" name="deputy_chief_arbiter" id="deputyChiefArbiter" />

                    <label htmlFor="timeControl">Control de tiempo</label>
                    <input type="text" name="time_control" id="timeControl" />

                    <label htmlFor="numberRounds">Número de rondas</label>
                    <input type="number" name="number_rounds" id="numberRounds" required />

                    <button type="submit">Crear</button>
                </form>
            </dialog>
        </>
    )
}

export default Home;
