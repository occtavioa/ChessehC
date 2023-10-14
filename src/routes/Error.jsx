import { Link } from "react-router-dom"

function Error() {
    return (
        <>
            <Link to={"/"}>Volver al inicio</Link>
            <p>Hubo un error</p>
        </>
    )
}

export default Error
