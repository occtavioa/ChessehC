import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { Alert, Badge, Button, FormSelect, Nav } from "react-bootstrap";
import { Link, Outlet, useHref, useNavigate, useParams } from "react-router-dom"

function TournamentLayout() {
    const {path, roundId} = useParams()
    const [rounds, setRounds] = useState([])
    const [selectedRoundId, setSelectedRoundId] = useState()
    const href = useHref()
    const navigate = useNavigate()
    const [makePairingResult, setMakePairingResult] = useState()
    
    useEffect(() => {
        invoke("get_rounds", {path: atob(path)})
            .then((rounds) => {
                if(rounds.length <= 0)
                    return
                setRounds(rounds)
                setSelectedRoundId(rounds.at(-1).id)
            })
            .catch(e => {
                console.error(e);
            })
        
    }, [path])
    
    useEffect(() => {
        console.log(selectedRoundId);
        if(roundId && selectedRoundId) {
            navigate(`round/${selectedRoundId}/${href.split("/").at(-1)}`)
        }
    }, [selectedRoundId])

    return (<>
        <Nav variant="tabs">
            <Nav.Item>
                <Nav.Link as={Link} to={"/"}>Inicio</Nav.Link>
            </Nav.Item>
            <Nav.Item>
                <Nav.Link as={Link} to={"."}>Torneo</Nav.Link>
            </Nav.Item>
            <Nav.Item>
                <Nav.Link as={Link} to={"players"}>Jugadores</Nav.Link>
            </Nav.Item>
            {
                rounds.length > 0 &&
                    <Nav.Item>
                        <FormSelect value={selectedRoundId} onChange={(e) => {setSelectedRoundId(parseInt(e.target.value))}}>
                            {
                                rounds.map(r => 
                                    <option value={r.id} key={r.id}>Ronda {r.number}</option>
                                )
                            }
                        </FormSelect>
                    </Nav.Item>
            }
            {
                selectedRoundId && (<>
                    <Nav.Item>
                        <Nav.Link as={Link} to={`round/${selectedRoundId}/pairings`}>Pareos</Nav.Link>
                    </Nav.Item>
                    <Nav.Item>
                        <Nav.Link as={Link} to={`round/${selectedRoundId}/standings`}>Clasificación</Nav.Link>
                    </Nav.Item>
                </>)
            }
            <Nav.Item>
                <Button onClick={async () => {
                    try {
                        await invoke("make_pairing", {path: atob(path)})
                        let rounds = await invoke("get_rounds", {path: atob(path)})
                        let currentRound = rounds.at(-1)
                        setRounds(rounds)
                        setSelectedRoundId(currentRound.id)
                        setMakePairingResult({type: "success", message: `Ronda ${currentRound.number} creada`})
                    } catch (error) {
                        console.error(error);
                        setMakePairingResult({type: "error", message: error})
                    }
                }}>Realizar pareo</Button>
            </Nav.Item>
        </Nav>
        {
            typeof makePairingResult !== "undefined"
            && (
            <Alert variant={makePairingResult.type === "success" ? "success" : "danger"}>
                {makePairingResult.message}
            </Alert>
        )
        }
        <Outlet></Outlet>
    </>)
}

export default TournamentLayout;
