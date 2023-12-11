import { invoke } from "@tauri-apps/api";
import { useEffect, useRef, useState } from "react";
import { Alert, Button, Form, Image, Modal } from "react-bootstrap";
import { useNavigate} from "react-router-dom";
import logo from "../assets/logo.png"

function Home() {
    const [showFormModal, setShowFormModal] = useState(false);
    const [createTournamentResult, setCreateTournamentResult] = useState();
    const [showCreateTournamentResult, setShowCreateTournamentResult] = useState(false);
    const [postTournamentResult, setPostTournamentResult] = useState();
    const [showPostTournamentResult, setShowPostTournamentResult] = useState(false);
    const navigate = useNavigate()

    useEffect(() => {
        setShowCreateTournamentResult(typeof createTournamentResult !== "undefined")
    }, [createTournamentResult])

    useEffect(() => {
        setShowPostTournamentResult(typeof postTournamentResult !== "undefined")
    }, [postTournamentResult])

    return (<>
        {
            showCreateTournamentResult
            && (
                <Alert variant={createTournamentResult.type}>
                    {createTournamentResult.message}
                </Alert>
            )
        }
        {
            showPostTournamentResult
            && (
                <Alert variant={postTournamentResult.type}>
                    {postTournamentResult.message}
                </Alert>
            )
        }

        <Image src={logo} alt="Logo"></Image>

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
                    tournament.id = 0;
                    let tournament_path = await invoke("save_tournament_file", {name: tournament.name})
                    if(!tournament_path) {
                        setCreateTournamentResult({type: "warning", message: "La creación del torneo fue cancelada"})
                        setShowFormModal(false)
                        return
                    }
                    try {
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
                        tournament.id = Id
                        setPostTournamentResult({type: "success", message: "El torneo se ha subido a la web"})
                    } catch (error) {
                        console.error(error);
                        setPostTournamentResult({type: "danger", message: "No se pudo subir el torneo a la web"})
                    }
                    try {
                        let path = await invoke("create_tournament", {tournament: tournament, path: tournament_path})
                        setCreateTournamentResult({type: "success", message: `Torneo creado en ${path}`})
                    } catch (error) {
                        console.error(error);
                        setCreateTournamentResult({type: "danger", message: "No se pudo crear el torneo"})
                    }
                    setShowFormModal(false)
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
