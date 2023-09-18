import { useEffect } from "react";
import { useParams } from "react-router";

function Standings() {
    const {path, round} = useParams()

    return (
        <>
            Standings
        </>
    )
}

export default Standings
