import { useEffect } from "react";
import { useLoaderData, useSearchParams } from "react-router-dom";

function TournamentData() {
    const tournament = useLoaderData()

    useEffect(() => {
        console.log(tournament);
    }, [tournament])
    
    return (
        <>
            Data
        </>
    )
}

export default TournamentData;