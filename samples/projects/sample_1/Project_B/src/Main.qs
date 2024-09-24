operation Teleport(msg : Qubit, target : Qubit) : Unit {
    Message("Project B: Running Teleport ...");
    use here = Qubit();

    PrepareBellPair(here, target);
    Adjoint PrepareBellPair(msg, here);

    if M(msg) == One { Z(target); }
    if M(here) == One { X(target); }

    Reset(here);
}

operation PrepareBellPair(left : Qubit, right : Qubit) : Unit is Adj + Ctl {
    H(left);
    CNOT(left, right);
}

export Teleport;       //  makes the Teleport operation available to external programs
