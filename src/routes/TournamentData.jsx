import { invoke } from "@tauri-apps/api";
import { useEffect } from "react";
import { useLoaderData, useNavigate, useParams } from "react-router-dom";

function TournamentData() {
    const {path} = useParams()
    const navigate = useNavigate()
    
    useEffect(() => {
        invoke("get_tournament", {path: atob(path)})
            .then((tournament) => {console.log(tournament);})
            .catch((error) => {console.error(error); navigate("/error")})
    }, [])
    
    return (
        <>
            Data
        </>
    )
}

export default TournamentData;
