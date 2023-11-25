import { invoke } from "@tauri-apps/api";
import { useRef, useState } from "react";
import { Button, Form, Modal } from "react-bootstrap";
import { useNavigate} from "react-router-dom";

function Home() {
    const formDialogRef = useRef(null);
    const [showFormModal, setShowFormModal] = useState(false);
    const navigate = useNavigate()

    return (
        <>
            <img src="" alt="chessehc-logo" />

            <Button onClick={() => {
                setShowFormModal(true)
            }}>Nuevo torneo</Button>

            <Button onClick={async () => {
                invoke("pick_tournament_file")
                    .then((path) => {if(path) navigate(`tournament/${btoa(path)}`)})
                    .catch((error) => {console.error(error)})
            }}>Cargar torneo</Button>

            <dialog ref={formDialogRef}>
                <button onClick={() => {
                    formDialogRef.current.close();
                }}>x</button>

                <form onSubmit={async (e) => {
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

            <Modal show={showFormModal} onHide={() => {setShowFormModal(false)}}>
                <Modal.Header closeButton>
                    <Modal.Title>Nuevo torneo</Modal.Title>
                </Modal.Header>
                <Modal.Body>
                    <Form onSubmit={async (e) => {
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
                        <Form.Group>
                            <Form.Label htmlFor="name">Nombre</Form.Label>
                            <Form.Control type="text" name="name" id="name" required/>
                        </Form.Group>
                        <Form.Group>
                            <Form.Label htmlFor="numberRounds">Número de rondas</Form.Label>
                            <Form.Control type="number" name="number_rounds" id="numberRounds" required min={5} max={9999}/>
                        </Form.Group>

                        <Button type="submit">Crear</Button>
                    </Form>
                </Modal.Body>
            </Modal>
        </>
    )
}

export default Home;
