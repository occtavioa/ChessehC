function getPointValue(p) {
    switch (p) {
        case "W":
            return 1.0
        case "D":
            return 1.0/2.0
        case "L":
            return 0.0
        case "U":
            return 1.0
        case "Z":
            return 0.0
        default:
            break;
    }
}
