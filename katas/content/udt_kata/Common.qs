namespace Kata {
    newtype ProtocolMessage = (Bit1 : Int, Bit2 : Int);
}

namespace Kata.Verification {
    open Kata;

    operation VerifyProtocolMessage(op : (ProtocolMessage => Int)) : Bool {
        Message($"Inside Proto Message");

        for n in 0..3 {
            let data = (n / 2, n % 2);

            let (dataBit1, dataBit2) = data;
            let expected = 2*dataBit1 + dataBit2;

            let message = ProtocolMessage(dataBit1, dataBit2);

            let actual = op(message);

            if expected != actual {
                Message($"Bit1 : {dataBit1}, bit2 : {dataBit2}");
                return false;
            }
        }
        Message("Correct");
        return true;
    }



}
