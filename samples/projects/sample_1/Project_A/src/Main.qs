import MyTeleportLib.Teleport;   // imports the Teleport operation from the MyTeleportLib namespace defined in the manifest file

operation Main() : Unit {
    use msg = Qubit();
    use target = Qubit();

    H(msg);
    Teleport(msg, target);    // calls the Teleport() operation from the MyTeleportLib namespace
    H(target);

    if M(target) == Zero {
        Message("Teleported successfully!");
        Reset(msg);
        Reset(target);
    }
}
