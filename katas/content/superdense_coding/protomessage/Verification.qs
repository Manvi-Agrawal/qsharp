namespace Kata.Verification {

    @EntryPoint()
    operation CheckSolution : Bool {
        Message("Inside check solution");
        return VerifyProtocolMessage(Kata.ProtoMessage);
    }
}
