// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

#![allow(
    clippy::needless_raw_string_hashes,
    clippy::similar_names,
    clippy::too_many_lines
)]

pub mod test_utils;

use expect_test::expect;
use indoc::indoc;
use qsc_rir::rir::{BlockId, CallableId};
use test_utils::{
    assert_block_instructions, assert_blocks, assert_callable, assert_error,
    get_partial_evaluation_error, get_rir_program,
};

#[test]
fn non_classical_entry_point_with_classical_implicit_return() {
    let program = get_rir_program(indoc! {r#"
        namespace Test {
            @EntryPoint()
            operation Main() : Bool {
                use q = Qubit(); // Needed to make `Main` non-classical.
                true
            }
        }
    "#});
    let output_recording_callable_id = CallableId(1);
    assert_callable(
        &program,
        output_recording_callable_id,
        &expect![[r#"
        Callable:
            name: __quantum__rt__bool_record_output
            call_type: OutputRecording
            input_type:
                [0]: Boolean
                [1]: Pointer
            output_type: <VOID>
            body: <NONE>"#]],
    );
    assert_block_instructions(
        &program,
        BlockId(0),
        &expect![[r#"
        Block:
            Call id(1), args( Bool(true), Pointer, )
            Return"#]],
    );
}

#[test]
fn non_classical_entry_point_with_non_classical_implicit_return() {
    let program = get_rir_program(indoc! {r#"
        namespace Test {
            @EntryPoint()
            operation Main() : Result {
                use q = Qubit();
                QIR.Intrinsic.__quantum__qis__mresetz__body(q)
            }
        }
    "#});
    let mresetz_callable_id = CallableId(1);
    assert_callable(
        &program,
        mresetz_callable_id,
        &expect![[r#"
            Callable:
                name: __quantum__qis__mresetz__body
                call_type: Measurement
                input_type:
                    [0]: Qubit
                    [1]: Result
                output_type: <VOID>
                body: <NONE>"#]],
    );
    let output_recording_callable_id = CallableId(2);
    assert_callable(
        &program,
        output_recording_callable_id,
        &expect![[r#"
            Callable:
                name: __quantum__rt__result_record_output
                call_type: OutputRecording
                input_type:
                    [0]: Result
                    [1]: Pointer
                output_type: <VOID>
                body: <NONE>"#]],
    );
    assert_block_instructions(
        &program,
        BlockId(0),
        &expect![[r#"
            Block:
                Call id(1), args( Qubit(0), Result(0), )
                Call id(2), args( Result(0), Pointer, )
                Return"#]],
    );
}

#[test]
fn non_classical_entry_point_with_classical_explicit_return() {
    let program = get_rir_program(indoc! {r#"
        namespace Test {
            @EntryPoint()
            operation Main() : Bool {
                use q = Qubit(); // Needed to make `Main` non-classical.
                return false;
            }
        }
    "#});
    let output_recording_callable_id = CallableId(1);
    assert_callable(
        &program,
        output_recording_callable_id,
        &expect![[r#"
        Callable:
            name: __quantum__rt__bool_record_output
            call_type: OutputRecording
            input_type:
                [0]: Boolean
                [1]: Pointer
            output_type: <VOID>
            body: <NONE>"#]],
    );
    assert_block_instructions(
        &program,
        BlockId(0),
        &expect![[r#"
        Block:
            Call id(1), args( Bool(false), Pointer, )
            Return"#]],
    );
}

#[test]
fn non_classical_entry_point_with_non_classical_explicit_return() {
    let program = get_rir_program(indoc! {r#"
        namespace Test {
            @EntryPoint()
            operation Main() : Result {
                use q = Qubit();
                return QIR.Intrinsic.__quantum__qis__mresetz__body(q);
            }
        }
    "#});
    let mresetz_callable_id = CallableId(1);
    assert_callable(
        &program,
        mresetz_callable_id,
        &expect![[r#"
            Callable:
                name: __quantum__qis__mresetz__body
                call_type: Measurement
                input_type:
                    [0]: Qubit
                    [1]: Result
                output_type: <VOID>
                body: <NONE>"#]],
    );
    let output_recording_callable_id = CallableId(2);
    assert_callable(
        &program,
        output_recording_callable_id,
        &expect![[r#"
            Callable:
                name: __quantum__rt__result_record_output
                call_type: OutputRecording
                input_type:
                    [0]: Result
                    [1]: Pointer
                output_type: <VOID>
                body: <NONE>"#]],
    );
    assert_block_instructions(
        &program,
        BlockId(0),
        &expect![[r#"
            Block:
                Call id(1), args( Qubit(0), Result(0), )
                Call id(2), args( Result(0), Pointer, )
                Return"#]],
    );
}

#[test]
fn non_classical_entry_point_with_classical_inline_early_return_halts_evaluation() {
    let program = get_rir_program(indoc! {r#"
    namespace Test {
        operation OpA(q : Qubit) : Unit { body intrinsic; }
        operation OpB(q : Qubit) : Unit { body intrinsic; }
        @EntryPoint()
        operation Main() : Unit {
            use q = Qubit();
            OpA(q);
            return ();
            OpB(q);
        }
    }
    "#});
    let op_a_callable_id = CallableId(1);
    assert_callable(
        &program,
        op_a_callable_id,
        &expect![[r#"
        Callable:
            name: OpA
            call_type: Regular
            input_type:
                [0]: Qubit
            output_type: <VOID>
            body: <NONE>"#]],
    );
    let output_recording_callable_id = CallableId(2);
    assert_callable(
        &program,
        output_recording_callable_id,
        &expect![[r#"
        Callable:
            name: __quantum__rt__tuple_record_output
            call_type: OutputRecording
            input_type:
                [0]: Integer
                [1]: Pointer
            output_type: <VOID>
            body: <NONE>"#]],
    );
    assert_block_instructions(
        &program,
        BlockId(0),
        &expect![[r#"
        Block:
            Call id(1), args( Qubit(0), )
            Call id(2), args( Integer(0), Pointer, )
            Return"#]],
    );
}

#[test]
fn non_classical_entry_point_with_non_classical_inline_early_return_halts_evaluation() {
    let program = get_rir_program(indoc! {r#"
        namespace Test {
            @EntryPoint()
            operation Main() : Result {
                use q = Qubit();
                return QIR.Intrinsic.__quantum__qis__mresetz__body(q);
                Zero
            }
        }
    "#});
    let mresetz_callable_id = CallableId(1);
    assert_callable(
        &program,
        mresetz_callable_id,
        &expect![[r#"
            Callable:
                name: __quantum__qis__mresetz__body
                call_type: Measurement
                input_type:
                    [0]: Qubit
                    [1]: Result
                output_type: <VOID>
                body: <NONE>"#]],
    );
    let output_recording_callable_id = CallableId(2);
    assert_callable(
        &program,
        output_recording_callable_id,
        &expect![[r#"
            Callable:
                name: __quantum__rt__result_record_output
                call_type: OutputRecording
                input_type:
                    [0]: Result
                    [1]: Pointer
                output_type: <VOID>
                body: <NONE>"#]],
    );
    assert_block_instructions(
        &program,
        BlockId(0),
        &expect![[r#"
            Block:
                Call id(1), args( Qubit(0), Result(0), )
                Call id(2), args( Result(0), Pointer, )
                Return"#]],
    );
}

#[test]
fn non_classical_entry_point_with_classical_early_return_within_classical_branch_halts_evaluation()
{
    let program = get_rir_program(indoc! {r#"
        namespace Test {
            operation OpA(q : Qubit) : Unit { body intrinsic; }
            operation OpB(q : Qubit) : Unit { body intrinsic; }
            @EntryPoint()
            operation Main() : Bool {
                use q = Qubit();
                if true {
                    OpA(q);
                    return true;
                }
                OpB(q);
                return false;
            }
        }
    "#});
    let op_a_callable_id = CallableId(1);
    assert_callable(
        &program,
        op_a_callable_id,
        &expect![[r#"
        Callable:
            name: OpA
            call_type: Regular
            input_type:
                [0]: Qubit
            output_type: <VOID>
            body: <NONE>"#]],
    );
    let output_recording_callable_id = CallableId(2);
    assert_callable(
        &program,
        output_recording_callable_id,
        &expect![[r#"
            Callable:
                name: __quantum__rt__bool_record_output
                call_type: OutputRecording
                input_type:
                    [0]: Boolean
                    [1]: Pointer
                output_type: <VOID>
                body: <NONE>"#]],
    );
    assert_block_instructions(
        &program,
        BlockId(0),
        &expect![[r#"
            Block:
                Call id(1), args( Qubit(0), )
                Call id(2), args( Bool(true), Pointer, )
                Return"#]],
    );
}

#[test]
fn non_classical_entry_point_with_classical_early_return_within_non_classical_branch_yields_error()
{
    let error = get_partial_evaluation_error(indoc! {r#"
        namespace Test {
            @EntryPoint()
            operation Main() : Bool {
                use q = Qubit();
                let r = QIR.Intrinsic.__quantum__qis__mresetz__body(q);
                if r == Zero {
                    return false;
                }
                return true;
            }
        }
    "#});
    assert_error(
        &error,
        &expect![[r#"Unexpected("early return", Span { lo: 163, hi: 213 })"#]],
    );
}

#[test]
fn non_classical_entry_point_with_non_classical_early_return_within_non_classical_branch_yields_error(
) {
    let error = get_partial_evaluation_error(indoc! {r#"
        namespace Test {
            @EntryPoint()
            operation Main() : Result {
                use (q0, q1) = (Qubit(), Qubit());
                let r0 = QIR.Intrinsic.__quantum__qis__mresetz__body(q0);
                if r0 == Zero {
                    return QIR.Intrinsic.__quantum__qis__mresetz__body(q1);
                }
                return One;
            }
        }
    "#});
    assert_error(
        &error,
        &expect![[r#"Unexpected("early return", Span { lo: 185, hi: 278 })"#]],
    );
}

#[test]
fn non_classical_entry_point_with_early_return_after_branching_halts_evaluation() {
    let program = get_rir_program(indoc! {r#"
        namespace Test {
            operation OpA(q : Qubit) : Unit { body intrinsic; }
            operation OpB(q : Qubit) : Unit { body intrinsic; }
            operation OpC(q : Qubit) : Unit { body intrinsic; }
            operation OpD(q : Qubit) : Unit { body intrinsic; }
            @EntryPoint()
            operation Main() : Unit {
                use (q0, q1) = (Qubit(), Qubit());
                let r0 = QIR.Intrinsic.__quantum__qis__mresetz__body(q0);
                if r0 == Zero {
                    OpA(q1);
                } else {
                    OpB(q1);
                }
                OpC(q1);
                return ();
                OpD(q1);
            }
        }
    "#});
    let mresetz_callable_id = CallableId(1);
    assert_callable(
        &program,
        mresetz_callable_id,
        &expect![[r#"
            Callable:
                name: __quantum__qis__mresetz__body
                call_type: Measurement
                input_type:
                    [0]: Qubit
                    [1]: Result
                output_type: <VOID>
                body: <NONE>"#]],
    );
    let read_result_callable_id = CallableId(2);
    assert_callable(
        &program,
        read_result_callable_id,
        &expect![[r#"
        Callable:
            name: __quantum__qis__read_result__body
            call_type: Readout
            input_type:
                [0]: Result
            output_type: Boolean
            body: <NONE>"#]],
    );
    let op_a_callable_id = CallableId(3);
    assert_callable(
        &program,
        op_a_callable_id,
        &expect![[r#"
        Callable:
            name: OpA
            call_type: Regular
            input_type:
                [0]: Qubit
            output_type: <VOID>
            body: <NONE>"#]],
    );
    let op_b_callable_id = CallableId(4);
    assert_callable(
        &program,
        op_b_callable_id,
        &expect![[r#"
        Callable:
            name: OpB
            call_type: Regular
            input_type:
                [0]: Qubit
            output_type: <VOID>
            body: <NONE>"#]],
    );
    let op_c_callable_id = CallableId(5);
    assert_callable(
        &program,
        op_c_callable_id,
        &expect![[r#"
        Callable:
            name: OpC
            call_type: Regular
            input_type:
                [0]: Qubit
            output_type: <VOID>
            body: <NONE>"#]],
    );
    let output_recording_callable_id = CallableId(6);
    assert_callable(
        &program,
        output_recording_callable_id,
        &expect![[r#"
            Callable:
                name: __quantum__rt__tuple_record_output
                call_type: OutputRecording
                input_type:
                    [0]: Integer
                    [1]: Pointer
                output_type: <VOID>
                body: <NONE>"#]],
    );
    assert_blocks(
        &program,
        &expect![[r#"
            Blocks:
            Block 0:Block:
                Call id(1), args( Qubit(0), Result(0), )
                Variable(0, Boolean) = Call id(2), args( Result(0), )
                Variable(1, Boolean) = Icmp Eq, Variable(0, Boolean), Bool(false)
                Branch Variable(1, Boolean), 2, 3
            Block 1:Block:
                Call id(5), args( Qubit(1), )
                Call id(6), args( Integer(0), Pointer, )
                Return
            Block 2:Block:
                Call id(3), args( Qubit(1), )
                Jump(1)
            Block 3:Block:
                Call id(4), args( Qubit(1), )
                Jump(1)"#]],
    );
}

#[test]
fn operation_with_early_return_within_dynamic_branch_halts_evaluation_at_the_callable_level() {
    let program = get_rir_program(indoc! {r#"
        namespace Test {
            operation OpA(q : Qubit) : Unit { body intrinsic; }
            operation OpB(q : Qubit) : Unit { body intrinsic; }
            operation OpC(q : Qubit) : Unit { body intrinsic; }
            operation EarlyReturn(q : Qubit) : Unit {
                OpA(q);
                return ();
                OpC(q);
            }
            @EntryPoint()
            operation Main() : Unit {
                use q = Qubit();
                let r = QIR.Intrinsic.__quantum__qis__mresetz__body(q);
                if r == Zero {
                    EarlyReturn(q);
                }
                OpB(q);
            }
        }
    "#});
    let mresetz_callable_id = CallableId(1);
    assert_callable(
        &program,
        mresetz_callable_id,
        &expect![[r#"
            Callable:
                name: __quantum__qis__mresetz__body
                call_type: Measurement
                input_type:
                    [0]: Qubit
                    [1]: Result
                output_type: <VOID>
                body: <NONE>"#]],
    );
    let read_result_callable_id = CallableId(2);
    assert_callable(
        &program,
        read_result_callable_id,
        &expect![[r#"
        Callable:
            name: __quantum__qis__read_result__body
            call_type: Readout
            input_type:
                [0]: Result
            output_type: Boolean
            body: <NONE>"#]],
    );
    let op_a_callable_id = CallableId(3);
    assert_callable(
        &program,
        op_a_callable_id,
        &expect![[r#"
        Callable:
            name: OpA
            call_type: Regular
            input_type:
                [0]: Qubit
            output_type: <VOID>
            body: <NONE>"#]],
    );
    let op_b_callable_id = CallableId(4);
    assert_callable(
        &program,
        op_b_callable_id,
        &expect![[r#"
        Callable:
            name: OpB
            call_type: Regular
            input_type:
                [0]: Qubit
            output_type: <VOID>
            body: <NONE>"#]],
    );
    let output_recording_callable_id = CallableId(5);
    assert_callable(
        &program,
        output_recording_callable_id,
        &expect![[r#"
            Callable:
                name: __quantum__rt__tuple_record_output
                call_type: OutputRecording
                input_type:
                    [0]: Integer
                    [1]: Pointer
                output_type: <VOID>
                body: <NONE>"#]],
    );
    assert_blocks(
        &program,
        &expect![[r#"
            Blocks:
            Block 0:Block:
                Call id(1), args( Qubit(0), Result(0), )
                Variable(0, Boolean) = Call id(2), args( Result(0), )
                Variable(1, Boolean) = Icmp Eq, Variable(0, Boolean), Bool(false)
                Branch Variable(1, Boolean), 2, 1
            Block 1:Block:
                Call id(4), args( Qubit(0), )
                Call id(5), args( Integer(0), Pointer, )
                Return
            Block 2:Block:
                Call id(3), args( Qubit(0), )
                Jump(1)"#]],
    );
}

#[test]
fn default_qubit_management_releases_qubits_when_they_are_out_of_scope_with_implicit_return() {
    let program = get_rir_program(indoc! {
        r#"
        namespace Test {
            operation AllocateAndApply() : Unit {
                use q = Qubit();
                OpB(q);
            }
            operation OpA(q : Qubit) : Unit { body intrinsic; }
            operation OpB(q : Qubit) : Unit { body intrinsic; }
            @EntryPoint()
            operation Main() : Unit {
                use q0 = Qubit();
                OpA(q0);
                AllocateAndApply();
                use q1 = Qubit();
                OpA(q1);
            }
        }
        "#,
    });
    let op_a_callable_id = CallableId(1);
    assert_callable(
        &program,
        op_a_callable_id,
        &expect![[r#"
        Callable:
            name: OpA
            call_type: Regular
            input_type:
                [0]: Qubit
            output_type: <VOID>
            body: <NONE>"#]],
    );
    let op_b_callable_id = CallableId(2);
    assert_callable(
        &program,
        op_b_callable_id,
        &expect![[r#"
        Callable:
            name: OpB
            call_type: Regular
            input_type:
                [0]: Qubit
            output_type: <VOID>
            body: <NONE>"#]],
    );
    assert_block_instructions(
        &program,
        BlockId(0),
        &expect![[r#"
        Block:
            Call id(1), args( Qubit(0), )
            Call id(2), args( Qubit(1), )
            Call id(1), args( Qubit(1), )
            Call id(3), args( Integer(0), Pointer, )
            Return"#]],
    );
    assert_eq!(program.num_qubits, 2);
    assert_eq!(program.num_results, 0);
}

#[test]
fn default_qubit_management_releases_qubits_when_they_are_out_of_scope_with_explicit_return() {
    let program = get_rir_program(indoc! {
        r#"
        namespace Test {
            operation AllocateAndApply() : Unit {
                use q = Qubit();
                OpB(q);
                return ();
            }
            operation OpA(q : Qubit) : Unit { body intrinsic; }
            operation OpB(q : Qubit) : Unit { body intrinsic; }
            @EntryPoint()
            operation Main() : Unit {
                use q0 = Qubit();
                OpA(q0);
                AllocateAndApply();
                use q1 = Qubit();
                OpA(q1);
            }
        }
        "#,
    });
    let op_a_callable_id = CallableId(1);
    assert_callable(
        &program,
        op_a_callable_id,
        &expect![[r#"
        Callable:
            name: OpA
            call_type: Regular
            input_type:
                [0]: Qubit
            output_type: <VOID>
            body: <NONE>"#]],
    );
    let op_b_callable_id = CallableId(2);
    assert_callable(
        &program,
        op_b_callable_id,
        &expect![[r#"
        Callable:
            name: OpB
            call_type: Regular
            input_type:
                [0]: Qubit
            output_type: <VOID>
            body: <NONE>"#]],
    );
    assert_block_instructions(
        &program,
        BlockId(0),
        &expect![[r#"
        Block:
            Call id(1), args( Qubit(0), )
            Call id(2), args( Qubit(1), )
            Call id(1), args( Qubit(1), )
            Call id(3), args( Integer(0), Pointer, )
            Return"#]],
    );
    assert_eq!(program.num_qubits, 2);
    assert_eq!(program.num_results, 0);
}

#[test]
fn default_qubit_management_releases_qubits_when_they_are_out_of_scope_with_explicit_early_return()
{
    let program = get_rir_program(indoc! {
        r#"
        namespace Test {
            operation AllocateAndApply() : Unit {
                use q = Qubit();
                OpB(q);
                return ();
                OpB(q);
            }
            operation OpA(q : Qubit) : Unit { body intrinsic; }
            operation OpB(q : Qubit) : Unit { body intrinsic; }
            @EntryPoint()
            operation Main() : Unit {
                use q0 = Qubit();
                OpA(q0);
                AllocateAndApply();
                use q1 = Qubit();
                OpA(q1);
            }
        }
        "#,
    });
    let op_a_callable_id = CallableId(1);
    assert_callable(
        &program,
        op_a_callable_id,
        &expect![[r#"
        Callable:
            name: OpA
            call_type: Regular
            input_type:
                [0]: Qubit
            output_type: <VOID>
            body: <NONE>"#]],
    );
    let op_b_callable_id = CallableId(2);
    assert_callable(
        &program,
        op_b_callable_id,
        &expect![[r#"
        Callable:
            name: OpB
            call_type: Regular
            input_type:
                [0]: Qubit
            output_type: <VOID>
            body: <NONE>"#]],
    );
    assert_block_instructions(
        &program,
        BlockId(0),
        &expect![[r#"
        Block:
            Call id(1), args( Qubit(0), )
            Call id(2), args( Qubit(1), )
            Call id(1), args( Qubit(1), )
            Call id(3), args( Integer(0), Pointer, )
            Return"#]],
    );
    assert_eq!(program.num_qubits, 2);
    assert_eq!(program.num_results, 0);
}

#[test]
fn explicit_return_embedded_in_array_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit(); // Needed to make `Main` non-classical.
            let a = [return false];
            true
        }
    }
    "#});
    assert_error(
        &error,
        &expect![[r#"Unexpected("embedded return in array", Span { lo: 148, hi: 160 })"#]],
    );
}

#[test]
fn explicit_return_embedded_in_array_repeat_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit(); // Needed to make `Main` non-classical.
            let a = [Zero, size = return false];
            true
        }
    }
    "#});
    assert_error(
        &error,
        &expect![[r#"Unexpected("embedded return in array size", Span { lo: 161, hi: 173 })"#]],
    );
}

#[test]
fn explicit_return_embedded_in_assign_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit(); // Needed to make `Main` non-classical.
            mutable b = true;
            set b = return false;
            b
        }
    }
    "#});
    assert_error(
        &error,
        &expect![[
            r#"Unexpected("embedded return in assign expression", Span { lo: 173, hi: 185 })"#
        ]],
    );
}

#[test]
fn explicit_return_embedded_in_assign_field_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        open Microsoft.Quantum.Math;
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit(); // Needed to make `Main` non-classical.
            let c1 = Complex(0.0, 1.0);
            let c2 = c1 w/ Real <- return false;
            true
        }
    }
    "#});
    // The type of error will change once this kind of hybrid expression is supported.
    assert_error(
        &error,
        &expect![[r#"Unimplemented("Updated Field Expr", Span { lo: 217, hi: 243 })"#]],
    );
}

#[test]
fn explicit_return_embedded_in_assign_index_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit(); // Needed to make `Main` non-classical.
            let a1 = [1];
            let a2 = a1 w/ 0 <- return false;
            true
        }
    }
    "#});
    // The type of error will change once this kind of hybrid expression is supported.
    assert_error(
        &error,
        &expect![[r#"Unimplemented("Update Index Expr", Span { lo: 170, hi: 193 })"#]],
    );
}

#[test]
fn explicit_return_embedded_in_assign_op_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit(); // Needed to make `Main` non-classical.
            mutable i = 0;
            set i += return false;
            true
        }
    }
    "#});
    // The type of error will change once this kind of hybrid expression is supported.
    assert_error(
        &error,
        &expect![[r#"Unimplemented("int binary operation", Span { lo: 166, hi: 167 })"#]],
    );
}

#[test]
fn explicit_return_embedded_in_bin_op_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit(); // Needed to make `Main` non-classical.
            let i = 0 * return false;
            true
        }
    }
    "#});
    assert_error(
        &error,
        &expect![[
            r#"Unexpected("embedded return in binary operation", Span { lo: 151, hi: 163 })"#
        ]],
    );
}

#[test]
fn explicit_return_embedded_in_call_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        open Microsoft.Quantum.Math;
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit(); // Needed to make `Main` non-classical.
            H(return false);
            true
        }
    }
    "#});
    assert_error(
        &error,
        &expect![[r#"Unexpected("embedded return in call arguments", Span { lo: 174, hi: 186 })"#]],
    );
}

#[test]
fn explicit_return_embedded_in_if_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        open Microsoft.Quantum.Math;
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit(); // Needed to make `Main` non-classical.
            if return false {
                X(q);
            }
            true
        }
    }
    "#});
    assert_error(
        &error,
        &expect![[r#"Unexpected("embedded return in if condition", Span { lo: 175, hi: 187 })"#]],
    );
}

#[test]
fn explicit_return_embedded_in_index_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit(); // Needed to make `Main` non-classical.
            let a = [1];
            let i = a[false ? 0 | return false];
            true
        }
    }
    "#});
    assert_error(
        &error,
        &expect![[
            r#"Unexpected("embedded return in index expression", Span { lo: 170, hi: 194 })"#
        ]],
    );
}

#[test]
fn explicit_return_embedded_in_tuple_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit(); // Needed to make `Main` non-classical.
            let a = (0, return false);
            true
        }
    }
    "#});
    assert_error(
        &error,
        &expect![[r#"Unexpected("embedded return in tuple", Span { lo: 151, hi: 163 })"#]],
    );
}

#[test]
fn explicit_return_embedded_in_unary_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit(); // Needed to make `Main` non-classical.
            let a = not return false;
            true
        }
    }
    "#});
    assert_error(
        &error,
        &expect![[
            r#"Unexpected("embedded return in unary operation expression", Span { lo: 151, hi: 163 })"#
        ]],
    );
}

#[test]
fn explicit_return_embedded_in_update_field_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        open Microsoft.Quantum.Math;
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit(); // Needed to make `Main` non-classical.
            mutable c = Complex(0.0, 1.0);
            set c w/= Real <- return false;
            true
        }
    }
    "#});
    // The type of error will change once this kind of hybrid expression is supported.
    assert_error(
        &error,
        &expect![[r#"Unimplemented("Field Assignment Expr", Span { lo: 211, hi: 241 })"#]],
    );
}

#[test]
fn explicit_return_embedded_in_hybrid_update_index_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit();
            mutable a = [true];
            set a w/= 0 <- return false;
            true
        }
    }
    "#});
    assert_error(
        &error,
        &expect![[
            r#"Unexpected("embedded return in assign index expression", Span { lo: 142, hi: 154 })"#
        ]],
    );
}

#[test]
fn explicit_return_embedded_in_hybrid_array_assignop_expr_yields_error() {
    let error = get_partial_evaluation_error(indoc! {r#"
    namespace Test {
        @EntryPoint()
        operation Main() : Bool {
            use q = Qubit();
            mutable a = [true];
            set a += return false;
            true
        }
    }
    "#});
    assert_error(
        &error,
        &expect![[r#"Unexpected("embedded return in RHS expression", Span { lo: 136, hi: 148 })"#]],
    );
}
