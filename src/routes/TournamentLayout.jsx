import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { Button, FormSelect, Nav } from "react-bootstrap";
import { Link, Outlet, useHref, useNavigate, useParams } from "react-router-dom"

function TournamentLayout() {
    const {path, roundId} = useParams()
    const [rounds, setRounds] = useState([])
    const [selectedRoundId, setSelectedRoundId] = useState()
    const href = useHref()
    const navigate = useNavigate()
    
    useEffect(() => {
        invoke("get_rounds", {path: atob(path)})
            .then((rounds) => {
                if(rounds.length > 0) {
                    setRounds(rounds)
                    setSelectedRoundId(rounds.at(-1).id)
                }
            })
            .catch(e => {
                console.error(e);
            })
        
    }, [path])
    
    useEffect(() => {
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
                    invoke("make_pairing", {path: atob(path)})
                        .then(() => {
                            invoke("get_rounds", {path: atob(path)})
                                .then((rounds) => {
                                    setRounds(rounds)
                                    if(!selectedRoundId) {
                                        setSelectedRoundId(rounds.at(-1).id)
                                    }
                                })
                                .catch(e => {
                                    console.error(e);
                                })
                        })
                        .catch((error) => {
                            console.error(error);
                        })
                }}>Realizar pareo</Button>
            </Nav.Item>
        </Nav>
        <Outlet></Outlet>
    </>)
}

export default TournamentLayout;
