import { invoke } from "@tauri-apps/api";
import { useRef, useState } from "react";
import { Button, Form, Modal } from "react-bootstrap";
import { useNavigate} from "react-router-dom";

function Home() {
    const [showFormModal, setShowFormModal] = useState(false);
    const navigate = useNavigate()

    return (<>
        <img src="" alt="chessehc-logo" />

        <Button onClick={() => {
            setShowFormModal(true)
        }}>Nuevo torneo</Button>

        <Button onClick={async () => {
            invoke("pick_tournament_file")
                .then((path) => {if(path) navigate(`tournament/${btoa(path)}`)})
                .catch((error) => {console.error(error)})
        }}>Cargar torneo</Button>

        <Modal show={showFormModal} onHide={() => {setShowFormModal(false)}}>
            <Modal.Header closeButton>
                <Modal.Title>Nuevo torneo</Modal.Title>
            </Modal.Header>
            <Modal.Body>
                <Form onSubmit={async (e) => {
                    e.preventDefault();

                    let tournament = Object.fromEntries(new FormData(e.target));
                    tournament.number_rounds = parseInt(tournament.number_rounds);
                    tournament.current_round_id = null;

                    const {Id} = await fetch("http://localhost:5000/tournaments", {
                        method: "POST",
                        headers: {
                            "Content-Type": "application/json"
                        },
                        body: JSON.stringify({
                            name: tournament.name,
                            numberOfRounds: tournament.number_rounds,
                            currentRound: null
                        })
                    })
                        .then((res) => res.json())
                        .catch((e) => {
                            console.error(e);
                            return 0;
                        })

                    tournament.id = Id;

                    invoke("create_tournament", {tournament: tournament})
                        .then((path) => {
                            if(path) {
                                console.log("Torneo creado en", path);
                            }
                        })
                        .catch((error) => {console.error(error);})
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
    </>)
}

export default Home;
