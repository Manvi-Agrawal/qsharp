namespace Kata {

    operation EncodeMessageInQubit(qAlice : Qubit, message : (Bool, Bool)) : Unit {
        // Get the bits from the message
        let (cbit1, cbit2) = message;

        if cbit2 {
            X(qAlice);
        }

        if cbit1 {
            Z(qAlice);
        }
    }
}