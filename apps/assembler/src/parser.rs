#![allow(non_snake_case)]
use crate::assembler::Assembler;
use crate::token::Token_Kind;
use byte_code::{Instruction, Register};
pub fn parse_instruction<'source>(asm: &mut Assembler<'source>) {
    if asm.matches(Token_Kind::Instruction) {
        let index = unsafe { asm.entry().value.integer };
        match index {
            0u64 => parse_Halt(asm),
            1u64 => parse_Trap(asm),
            2u64 => parse_Call(asm),
            3u64 => parse_Call_R(asm),
            4u64 => parse_Call_I(asm),
            5u64 => parse_Ret(asm),
            6u64 => parse_Ecall(asm),
            7u64 => parse_Break(asm),
            8u64 => parse_Jal(asm),
            9u64 => parse_Jal_R(asm),
            10u64 => parse_Jal_I(asm),
            11u64 => parse_Jnz(asm),
            12u64 => parse_Jnz_R(asm),
            13u64 => parse_Jnz_I(asm),
            14u64 => parse_Jiz(asm),
            15u64 => parse_Jiz_R(asm),
            16u64 => parse_Jiz_I(asm),
            17u64 => parse_Load_8(asm),
            18u64 => parse_Load_16(asm),
            19u64 => parse_Load_32(asm),
            20u64 => parse_Load_64(asm),
            21u64 => parse_Load_I(asm),
            22u64 => parse_Load_A_8(asm),
            23u64 => parse_Load_A_16(asm),
            24u64 => parse_Load_A_32(asm),
            25u64 => parse_Load_A_64(asm),
            26u64 => parse_Store_8(asm),
            27u64 => parse_Store_16(asm),
            28u64 => parse_Store_32(asm),
            29u64 => parse_Store_64(asm),
            30u64 => parse_Store_I(asm),
            31u64 => parse_Move(asm),
            32u64 => parse_Push(asm),
            33u64 => parse_Push_I(asm),
            34u64 => parse_Pop(asm),
            35u64 => parse_Ie(asm),
            36u64 => parse_Ie_f32(asm),
            37u64 => parse_Ie_f64(asm),
            38u64 => parse_Ne(asm),
            39u64 => parse_Ne_f32(asm),
            40u64 => parse_Ne_f64(asm),
            41u64 => parse_Lt(asm),
            42u64 => parse_Lt_f32(asm),
            43u64 => parse_Lt_f64(asm),
            44u64 => parse_Le(asm),
            45u64 => parse_Le_f32(asm),
            46u64 => parse_Le_f64(asm),
            47u64 => parse_Gt(asm),
            48u64 => parse_Gt_f32(asm),
            49u64 => parse_Gt_f64(asm),
            50u64 => parse_Ge(asm),
            51u64 => parse_Ge_f32(asm),
            52u64 => parse_Ge_f64(asm),
            53u64 => parse_And_i8(asm),
            54u64 => parse_And_i16(asm),
            55u64 => parse_And_i32(asm),
            56u64 => parse_And_i64(asm),
            57u64 => parse_And_u8(asm),
            58u64 => parse_And_u16(asm),
            59u64 => parse_And_u32(asm),
            60u64 => parse_And_u64(asm),
            61u64 => parse_Or_i8(asm),
            62u64 => parse_Or_i16(asm),
            63u64 => parse_Or_i32(asm),
            64u64 => parse_Or_i64(asm),
            65u64 => parse_Or_u8(asm),
            66u64 => parse_Or_u16(asm),
            67u64 => parse_Or_u32(asm),
            68u64 => parse_Or_u64(asm),
            69u64 => parse_Xor_i8(asm),
            70u64 => parse_Xor_i16(asm),
            71u64 => parse_Xor_i32(asm),
            72u64 => parse_Xor_i64(asm),
            73u64 => parse_Xor_u8(asm),
            74u64 => parse_Xor_u16(asm),
            75u64 => parse_Xor_u32(asm),
            76u64 => parse_Xor_u64(asm),
            77u64 => parse_Not_i8(asm),
            78u64 => parse_Not_i16(asm),
            79u64 => parse_Not_i32(asm),
            80u64 => parse_Not_i64(asm),
            81u64 => parse_Not_u8(asm),
            82u64 => parse_Not_u16(asm),
            83u64 => parse_Not_u32(asm),
            84u64 => parse_Not_u64(asm),
            85u64 => parse_Shl_i8(asm),
            86u64 => parse_Shl_i16(asm),
            87u64 => parse_Shl_i32(asm),
            88u64 => parse_Shl_i64(asm),
            89u64 => parse_Shl_u8(asm),
            90u64 => parse_Shl_u16(asm),
            91u64 => parse_Shl_u32(asm),
            92u64 => parse_Shl_u64(asm),
            93u64 => parse_Shr_i8(asm),
            94u64 => parse_Shr_i16(asm),
            95u64 => parse_Shr_i32(asm),
            96u64 => parse_Shr_i64(asm),
            97u64 => parse_Shr_u8(asm),
            98u64 => parse_Shr_u16(asm),
            99u64 => parse_Shr_u32(asm),
            100u64 => parse_Shr_u64(asm),
            101u64 => parse_Rot_L_i8(asm),
            102u64 => parse_Rot_L_i16(asm),
            103u64 => parse_Rot_L_i32(asm),
            104u64 => parse_Rot_L_i64(asm),
            105u64 => parse_Rot_L_u8(asm),
            106u64 => parse_Rot_L_u16(asm),
            107u64 => parse_Rot_L_u32(asm),
            108u64 => parse_Rot_L_u64(asm),
            109u64 => parse_Rot_R_i8(asm),
            110u64 => parse_Rot_R_i16(asm),
            111u64 => parse_Rot_R_i32(asm),
            112u64 => parse_Rot_R_i64(asm),
            113u64 => parse_Rot_R_u8(asm),
            114u64 => parse_Rot_R_u16(asm),
            115u64 => parse_Rot_R_u32(asm),
            116u64 => parse_Rot_R_u64(asm),
            117u64 => parse_C_Ones_i8(asm),
            118u64 => parse_C_Ones_i16(asm),
            119u64 => parse_C_Ones_i32(asm),
            120u64 => parse_C_Ones_i64(asm),
            121u64 => parse_C_Ones_u8(asm),
            122u64 => parse_C_Ones_u16(asm),
            123u64 => parse_C_Ones_u32(asm),
            124u64 => parse_C_Ones_u64(asm),
            125u64 => parse_L_Ones_i8(asm),
            126u64 => parse_L_Ones_i16(asm),
            127u64 => parse_L_Ones_i32(asm),
            128u64 => parse_L_Ones_i64(asm),
            129u64 => parse_L_Ones_u8(asm),
            130u64 => parse_L_Ones_u16(asm),
            131u64 => parse_L_Ones_u32(asm),
            132u64 => parse_L_Ones_u64(asm),
            133u64 => parse_T_Ones_i8(asm),
            134u64 => parse_T_Ones_i16(asm),
            135u64 => parse_T_Ones_i32(asm),
            136u64 => parse_T_Ones_i64(asm),
            137u64 => parse_T_Ones_u8(asm),
            138u64 => parse_T_Ones_u16(asm),
            139u64 => parse_T_Ones_u32(asm),
            140u64 => parse_T_Ones_u64(asm),
            141u64 => parse_C_Zeros_i8(asm),
            142u64 => parse_C_Zeros_i16(asm),
            143u64 => parse_C_Zeros_i32(asm),
            144u64 => parse_C_Zeros_i64(asm),
            145u64 => parse_C_Zeros_u8(asm),
            146u64 => parse_C_Zeros_u16(asm),
            147u64 => parse_C_Zeros_u32(asm),
            148u64 => parse_C_Zeros_u64(asm),
            149u64 => parse_L_Zeros_i8(asm),
            150u64 => parse_L_Zeros_i16(asm),
            151u64 => parse_L_Zeros_i32(asm),
            152u64 => parse_L_Zeros_i64(asm),
            153u64 => parse_L_Zeros_u8(asm),
            154u64 => parse_L_Zeros_u16(asm),
            155u64 => parse_L_Zeros_u32(asm),
            156u64 => parse_L_Zeros_u64(asm),
            157u64 => parse_T_Zeros_i8(asm),
            158u64 => parse_T_Zeros_i16(asm),
            159u64 => parse_T_Zeros_i32(asm),
            160u64 => parse_T_Zeros_i64(asm),
            161u64 => parse_T_Zeros_u8(asm),
            162u64 => parse_T_Zeros_u16(asm),
            163u64 => parse_T_Zeros_u32(asm),
            164u64 => parse_T_Zeros_u64(asm),
            165u64 => parse_R_Bytes_i8(asm),
            166u64 => parse_R_Bytes_i16(asm),
            167u64 => parse_R_Bytes_i32(asm),
            168u64 => parse_R_Bytes_i64(asm),
            169u64 => parse_R_Bytes_u8(asm),
            170u64 => parse_R_Bytes_u16(asm),
            171u64 => parse_R_Bytes_u32(asm),
            172u64 => parse_R_Bytes_u64(asm),
            173u64 => parse_R_Bits_i8(asm),
            174u64 => parse_R_Bits_i16(asm),
            175u64 => parse_R_Bits_i32(asm),
            176u64 => parse_R_Bits_i64(asm),
            177u64 => parse_R_Bits_u8(asm),
            178u64 => parse_R_Bits_u16(asm),
            179u64 => parse_R_Bits_u32(asm),
            180u64 => parse_R_Bits_u64(asm),
            181u64 => parse_C_Abs_i8(asm),
            182u64 => parse_C_Abs_i16(asm),
            183u64 => parse_C_Abs_i32(asm),
            184u64 => parse_C_Abs_i64(asm),
            185u64 => parse_C_Add_i8(asm),
            186u64 => parse_C_Add_i16(asm),
            187u64 => parse_C_Add_i32(asm),
            188u64 => parse_C_Add_i64(asm),
            189u64 => parse_C_Add_u8(asm),
            190u64 => parse_C_Add_u16(asm),
            191u64 => parse_C_Add_u32(asm),
            192u64 => parse_C_Add_u64(asm),
            193u64 => parse_C_Add_U_i8(asm),
            194u64 => parse_C_Add_U_i16(asm),
            195u64 => parse_C_Add_U_i32(asm),
            196u64 => parse_C_Add_U_i64(asm),
            197u64 => parse_C_Add_S_u8(asm),
            198u64 => parse_C_Add_S_u16(asm),
            199u64 => parse_C_Add_S_u32(asm),
            200u64 => parse_C_Add_S_u64(asm),
            201u64 => parse_C_Div_i8(asm),
            202u64 => parse_C_Div_i16(asm),
            203u64 => parse_C_Div_i32(asm),
            204u64 => parse_C_Div_i64(asm),
            205u64 => parse_C_Div_u8(asm),
            206u64 => parse_C_Div_u16(asm),
            207u64 => parse_C_Div_u32(asm),
            208u64 => parse_C_Div_u64(asm),
            209u64 => parse_C_Div_E_i8(asm),
            210u64 => parse_C_Div_E_i16(asm),
            211u64 => parse_C_Div_E_i32(asm),
            212u64 => parse_C_Div_E_i64(asm),
            213u64 => parse_C_Div_E_u8(asm),
            214u64 => parse_C_Div_E_u16(asm),
            215u64 => parse_C_Div_E_u32(asm),
            216u64 => parse_C_Div_E_u64(asm),
            217u64 => parse_C_Log_i8(asm),
            218u64 => parse_C_Log_i16(asm),
            219u64 => parse_C_Log_i32(asm),
            220u64 => parse_C_Log_i64(asm),
            221u64 => parse_C_Log_u8(asm),
            222u64 => parse_C_Log_u16(asm),
            223u64 => parse_C_Log_u32(asm),
            224u64 => parse_C_Log_u64(asm),
            225u64 => parse_C_Sqrt_i8(asm),
            226u64 => parse_C_Sqrt_i16(asm),
            227u64 => parse_C_Sqrt_i32(asm),
            228u64 => parse_C_Sqrt_i64(asm),
            229u64 => parse_C_Sqrt_u8(asm),
            230u64 => parse_C_Sqrt_u16(asm),
            231u64 => parse_C_Sqrt_u32(asm),
            232u64 => parse_C_Sqrt_u64(asm),
            233u64 => parse_C_Mul_i8(asm),
            234u64 => parse_C_Mul_i16(asm),
            235u64 => parse_C_Mul_i32(asm),
            236u64 => parse_C_Mul_i64(asm),
            237u64 => parse_C_Mul_u8(asm),
            238u64 => parse_C_Mul_u16(asm),
            239u64 => parse_C_Mul_u32(asm),
            240u64 => parse_C_Mul_u64(asm),
            241u64 => parse_C_Neg_i8(asm),
            242u64 => parse_C_Neg_i16(asm),
            243u64 => parse_C_Neg_i32(asm),
            244u64 => parse_C_Neg_i64(asm),
            245u64 => parse_C_Pow_i8(asm),
            246u64 => parse_C_Pow_i16(asm),
            247u64 => parse_C_Pow_i32(asm),
            248u64 => parse_C_Pow_i64(asm),
            249u64 => parse_C_Pow_u8(asm),
            250u64 => parse_C_Pow_u16(asm),
            251u64 => parse_C_Pow_u32(asm),
            252u64 => parse_C_Pow_u64(asm),
            253u64 => parse_C_Rem_i8(asm),
            254u64 => parse_C_Rem_i16(asm),
            255u64 => parse_C_Rem_i32(asm),
            256u64 => parse_C_Rem_i64(asm),
            257u64 => parse_C_Rem_u8(asm),
            258u64 => parse_C_Rem_u16(asm),
            259u64 => parse_C_Rem_u32(asm),
            260u64 => parse_C_Rem_u64(asm),
            261u64 => parse_C_Rem_E_i8(asm),
            262u64 => parse_C_Rem_E_i16(asm),
            263u64 => parse_C_Rem_E_i32(asm),
            264u64 => parse_C_Rem_E_i64(asm),
            265u64 => parse_C_Rem_E_u8(asm),
            266u64 => parse_C_Rem_E_u16(asm),
            267u64 => parse_C_Rem_E_u32(asm),
            268u64 => parse_C_Rem_E_u64(asm),
            269u64 => parse_C_Shl_i8(asm),
            270u64 => parse_C_Shl_i16(asm),
            271u64 => parse_C_Shl_i32(asm),
            272u64 => parse_C_Shl_i64(asm),
            273u64 => parse_C_Shl_u8(asm),
            274u64 => parse_C_Shl_u16(asm),
            275u64 => parse_C_Shl_u32(asm),
            276u64 => parse_C_Shl_u64(asm),
            277u64 => parse_C_Shr_i8(asm),
            278u64 => parse_C_Shr_i16(asm),
            279u64 => parse_C_Shr_i32(asm),
            280u64 => parse_C_Shr_i64(asm),
            281u64 => parse_C_Shr_u8(asm),
            282u64 => parse_C_Shr_u16(asm),
            283u64 => parse_C_Shr_u32(asm),
            284u64 => parse_C_Shr_u64(asm),
            285u64 => parse_C_Sub_i8(asm),
            286u64 => parse_C_Sub_i16(asm),
            287u64 => parse_C_Sub_i32(asm),
            288u64 => parse_C_Sub_i64(asm),
            289u64 => parse_C_Sub_u8(asm),
            290u64 => parse_C_Sub_u16(asm),
            291u64 => parse_C_Sub_u32(asm),
            292u64 => parse_C_Sub_u64(asm),
            293u64 => parse_C_Sub_U_i8(asm),
            294u64 => parse_C_Sub_U_i16(asm),
            295u64 => parse_C_Sub_U_i32(asm),
            296u64 => parse_C_Sub_U_i64(asm),
            297u64 => parse_O_Abs_i8(asm),
            298u64 => parse_O_Abs_i16(asm),
            299u64 => parse_O_Abs_i32(asm),
            300u64 => parse_O_Abs_i64(asm),
            301u64 => parse_O_Add_i8(asm),
            302u64 => parse_O_Add_i16(asm),
            303u64 => parse_O_Add_i32(asm),
            304u64 => parse_O_Add_i64(asm),
            305u64 => parse_O_Add_u8(asm),
            306u64 => parse_O_Add_u16(asm),
            307u64 => parse_O_Add_u32(asm),
            308u64 => parse_O_Add_u64(asm),
            309u64 => parse_O_Add_U_i8(asm),
            310u64 => parse_O_Add_U_i16(asm),
            311u64 => parse_O_Add_U_i32(asm),
            312u64 => parse_O_Add_U_i64(asm),
            313u64 => parse_O_Add_S_u8(asm),
            314u64 => parse_O_Add_S_u16(asm),
            315u64 => parse_O_Add_S_u32(asm),
            316u64 => parse_O_Add_S_u64(asm),
            317u64 => parse_O_Div_i8(asm),
            318u64 => parse_O_Div_i16(asm),
            319u64 => parse_O_Div_i32(asm),
            320u64 => parse_O_Div_i64(asm),
            321u64 => parse_O_Div_u8(asm),
            322u64 => parse_O_Div_u16(asm),
            323u64 => parse_O_Div_u32(asm),
            324u64 => parse_O_Div_u64(asm),
            325u64 => parse_O_Div_E_i8(asm),
            326u64 => parse_O_Div_E_i16(asm),
            327u64 => parse_O_Div_E_i32(asm),
            328u64 => parse_O_Div_E_i64(asm),
            329u64 => parse_O_Div_E_u8(asm),
            330u64 => parse_O_Div_E_u16(asm),
            331u64 => parse_O_Div_E_u32(asm),
            332u64 => parse_O_Div_E_u64(asm),
            333u64 => parse_O_Mul_i8(asm),
            334u64 => parse_O_Mul_i16(asm),
            335u64 => parse_O_Mul_i32(asm),
            336u64 => parse_O_Mul_i64(asm),
            337u64 => parse_O_Mul_u8(asm),
            338u64 => parse_O_Mul_u16(asm),
            339u64 => parse_O_Mul_u32(asm),
            340u64 => parse_O_Mul_u64(asm),
            341u64 => parse_O_Neg_i8(asm),
            342u64 => parse_O_Neg_i16(asm),
            343u64 => parse_O_Neg_i32(asm),
            344u64 => parse_O_Neg_i64(asm),
            345u64 => parse_O_Pow_i8(asm),
            346u64 => parse_O_Pow_i16(asm),
            347u64 => parse_O_Pow_i32(asm),
            348u64 => parse_O_Pow_i64(asm),
            349u64 => parse_O_Pow_u8(asm),
            350u64 => parse_O_Pow_u16(asm),
            351u64 => parse_O_Pow_u32(asm),
            352u64 => parse_O_Pow_u64(asm),
            353u64 => parse_O_Rem_i8(asm),
            354u64 => parse_O_Rem_i16(asm),
            355u64 => parse_O_Rem_i32(asm),
            356u64 => parse_O_Rem_i64(asm),
            357u64 => parse_O_Rem_u8(asm),
            358u64 => parse_O_Rem_u16(asm),
            359u64 => parse_O_Rem_u32(asm),
            360u64 => parse_O_Rem_u64(asm),
            361u64 => parse_O_Rem_E_i8(asm),
            362u64 => parse_O_Rem_E_i16(asm),
            363u64 => parse_O_Rem_E_i32(asm),
            364u64 => parse_O_Rem_E_i64(asm),
            365u64 => parse_O_Rem_E_u8(asm),
            366u64 => parse_O_Rem_E_u16(asm),
            367u64 => parse_O_Rem_E_u32(asm),
            368u64 => parse_O_Rem_E_u64(asm),
            369u64 => parse_O_Shl_i8(asm),
            370u64 => parse_O_Shl_i16(asm),
            371u64 => parse_O_Shl_i32(asm),
            372u64 => parse_O_Shl_i64(asm),
            373u64 => parse_O_Shl_u8(asm),
            374u64 => parse_O_Shl_u16(asm),
            375u64 => parse_O_Shl_u32(asm),
            376u64 => parse_O_Shl_u64(asm),
            377u64 => parse_O_Shr_i8(asm),
            378u64 => parse_O_Shr_i16(asm),
            379u64 => parse_O_Shr_i32(asm),
            380u64 => parse_O_Shr_i64(asm),
            381u64 => parse_O_Shr_u8(asm),
            382u64 => parse_O_Shr_u16(asm),
            383u64 => parse_O_Shr_u32(asm),
            384u64 => parse_O_Shr_u64(asm),
            385u64 => parse_O_Sub_i8(asm),
            386u64 => parse_O_Sub_i16(asm),
            387u64 => parse_O_Sub_i32(asm),
            388u64 => parse_O_Sub_i64(asm),
            389u64 => parse_O_Sub_u8(asm),
            390u64 => parse_O_Sub_u16(asm),
            391u64 => parse_O_Sub_u32(asm),
            392u64 => parse_O_Sub_u64(asm),
            393u64 => parse_O_Sub_U_i8(asm),
            394u64 => parse_O_Sub_U_i16(asm),
            395u64 => parse_O_Sub_U_i32(asm),
            396u64 => parse_O_Sub_U_i64(asm),
            397u64 => parse_S_Abs_i8(asm),
            398u64 => parse_S_Abs_i16(asm),
            399u64 => parse_S_Abs_i32(asm),
            400u64 => parse_S_Abs_i64(asm),
            401u64 => parse_S_Add_i8(asm),
            402u64 => parse_S_Add_i16(asm),
            403u64 => parse_S_Add_i32(asm),
            404u64 => parse_S_Add_i64(asm),
            405u64 => parse_S_Add_u8(asm),
            406u64 => parse_S_Add_u16(asm),
            407u64 => parse_S_Add_u32(asm),
            408u64 => parse_S_Add_u64(asm),
            409u64 => parse_S_Add_U_i8(asm),
            410u64 => parse_S_Add_U_i16(asm),
            411u64 => parse_S_Add_U_i32(asm),
            412u64 => parse_S_Add_U_i64(asm),
            413u64 => parse_S_Add_S_u8(asm),
            414u64 => parse_S_Add_S_u16(asm),
            415u64 => parse_S_Add_S_u32(asm),
            416u64 => parse_S_Add_S_u64(asm),
            417u64 => parse_S_Div_i8(asm),
            418u64 => parse_S_Div_i16(asm),
            419u64 => parse_S_Div_i32(asm),
            420u64 => parse_S_Div_i64(asm),
            421u64 => parse_S_Div_u8(asm),
            422u64 => parse_S_Div_u16(asm),
            423u64 => parse_S_Div_u32(asm),
            424u64 => parse_S_Div_u64(asm),
            425u64 => parse_S_Mul_i8(asm),
            426u64 => parse_S_Mul_i16(asm),
            427u64 => parse_S_Mul_i32(asm),
            428u64 => parse_S_Mul_i64(asm),
            429u64 => parse_S_Mul_u8(asm),
            430u64 => parse_S_Mul_u16(asm),
            431u64 => parse_S_Mul_u32(asm),
            432u64 => parse_S_Mul_u64(asm),
            433u64 => parse_S_Neg_i8(asm),
            434u64 => parse_S_Neg_i16(asm),
            435u64 => parse_S_Neg_i32(asm),
            436u64 => parse_S_Neg_i64(asm),
            437u64 => parse_S_Pow_i8(asm),
            438u64 => parse_S_Pow_i16(asm),
            439u64 => parse_S_Pow_i32(asm),
            440u64 => parse_S_Pow_i64(asm),
            441u64 => parse_S_Pow_u8(asm),
            442u64 => parse_S_Pow_u16(asm),
            443u64 => parse_S_Pow_u32(asm),
            444u64 => parse_S_Pow_u64(asm),
            445u64 => parse_S_Sub_i8(asm),
            446u64 => parse_S_Sub_i16(asm),
            447u64 => parse_S_Sub_i32(asm),
            448u64 => parse_S_Sub_i64(asm),
            449u64 => parse_S_Sub_u8(asm),
            450u64 => parse_S_Sub_u16(asm),
            451u64 => parse_S_Sub_u32(asm),
            452u64 => parse_S_Sub_u64(asm),
            453u64 => parse_S_Sub_U_i8(asm),
            454u64 => parse_S_Sub_U_i16(asm),
            455u64 => parse_S_Sub_U_i32(asm),
            456u64 => parse_S_Sub_U_i64(asm),
            457u64 => parse_Abs_f32(asm),
            458u64 => parse_Abs_f64(asm),
            459u64 => parse_Add_f32(asm),
            460u64 => parse_Add_f64(asm),
            461u64 => parse_Div_f32(asm),
            462u64 => parse_Div_f64(asm),
            463u64 => parse_Div_E_f32(asm),
            464u64 => parse_Div_E_f64(asm),
            465u64 => parse_Log_f32(asm),
            466u64 => parse_Log_f64(asm),
            467u64 => parse_Sqrt_f32(asm),
            468u64 => parse_Sqrt_f64(asm),
            469u64 => parse_Mul_f32(asm),
            470u64 => parse_Mul_f64(asm),
            471u64 => parse_Neg_f32(asm),
            472u64 => parse_Neg_f64(asm),
            473u64 => parse_Pow_f32(asm),
            474u64 => parse_Pow_f64(asm),
            475u64 => parse_Rem_f32(asm),
            476u64 => parse_Rem_f64(asm),
            477u64 => parse_Rem_E_f32(asm),
            478u64 => parse_Rem_E_f64(asm),
            479u64 => parse_Cbrt_f32(asm),
            480u64 => parse_Cbrt_f64(asm),
            481u64 => parse_Sub_f32(asm),
            482u64 => parse_Sub_f64(asm),
            _ => unreachable!(),
        }
    }
}
#[inline(always)]
fn parse_register<'source>(asm: &mut Assembler<'source>) -> Register {
    asm.expects(Token_Kind::Register);
    let index = unsafe { asm.entry().value.integer };
    Register::from_index(index as u8)
}
#[inline(always)]
fn parse_register_comma<'source>(asm: &mut Assembler<'source>) -> Register {
    asm.expects(Token_Kind::Register);
    let index = unsafe { asm.entry().value.integer };
    asm.expects(Token_Kind::Comma);
    Register::from_index(index as u8)
}
#[inline(always)]
fn parse_size<'source>(asm: &mut Assembler<'source>) -> u8 {
    asm.expects(Token_Kind::Type);
    unsafe { asm.entry().value.integer as u8 }
}
#[inline(always)]
fn parse_imm<'source>(asm: &mut Assembler<'source>) -> i16 {
    if asm.matches(Token_Kind::Colon) {
        asm.expects(Token_Kind::Identifier);
        asm.patch_label()
    } else if asm.matches(Token_Kind::Ampersand) {
        asm.expects(Token_Kind::Identifier);
        asm.patch_address()
    } else {
        asm.expects(Token_Kind::Number);
        unsafe { std::mem::transmute::<u64, i64>(asm.entry().value.integer) as i16 }
    }
}
fn parse_Halt<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Halt;
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Trap<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Trap;
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Call<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Call {
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Call_R<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Call_R {
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Call_I<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Call_I {
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Ret<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ret;
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Ecall<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ecall {
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Break<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Break;
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Jal<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jal {
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Jal_R<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jal_R {
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Jal_I<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jal_I {
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Jnz<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jnz {
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Jnz_R<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jnz_R {
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Jnz_I<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jnz_I {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Jiz<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jiz {
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Jiz_R<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jiz_R {
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Jiz_I<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jiz_I {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Load_8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Load_8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Load_16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Load_16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Load_32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Load_32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Load_64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Load_64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Load_I<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Load_I {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Load_A_8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Load_A_8 {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Load_A_16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Load_A_16 {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Load_A_32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Load_A_32 {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Load_A_64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Load_A_64 {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Store_8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Store_8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Store_16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Store_16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Store_32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Store_32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Store_64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Store_64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Store_I<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Store_I {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Move<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Move {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Push<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Push {
        rd: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Push_I<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Push_I {
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Pop<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Pop {
        rd: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Ie<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ie {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Ie_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ie_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Ie_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ie_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Ne<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ne {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Ne_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ne_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Ne_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ne_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Lt<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Lt {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Lt_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Lt_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Lt_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Lt_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Le<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Le {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Le_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Le_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Le_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Le_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Gt<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Gt {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Gt_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Gt_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Gt_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Gt_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Ge<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ge {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Ge_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ge_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Ge_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ge_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_And_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_And_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_And_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_And_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_And_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_And_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_And_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_And_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Or_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Or_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Or_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Or_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Or_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Or_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Or_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Or_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Xor_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Xor_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Xor_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Xor_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Xor_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Xor_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Xor_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Xor_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Not_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Not_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Not_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Not_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Not_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Not_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Not_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Not_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shl_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shl_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shl_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shl_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shl_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shl_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shl_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shl_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shr_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shr_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shr_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shr_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shr_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shr_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shr_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Shr_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_L_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_L_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_L_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_L_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_L_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_L_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_L_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_L_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_R_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_R_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_R_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_R_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_R_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_R_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_R_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rot_R_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Ones_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Ones_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Ones_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Ones_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Ones_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Ones_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Ones_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Ones_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Ones_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Ones_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Ones_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Ones_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Ones_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Ones_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Ones_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Ones_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Ones_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Ones_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Ones_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Ones_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Ones_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Ones_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Ones_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Ones_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Zeros_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Zeros_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Zeros_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Zeros_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Zeros_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Zeros_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Zeros_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Zeros_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Zeros_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Zeros_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Zeros_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Zeros_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Zeros_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Zeros_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Zeros_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_L_Zeros_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Zeros_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Zeros_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Zeros_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Zeros_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Zeros_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Zeros_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Zeros_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_T_Zeros_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bytes_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bytes_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bytes_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bytes_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bytes_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bytes_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bytes_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bytes_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bits_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bits_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bits_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bits_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bits_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bits_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bits_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_R_Bits_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Abs_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Abs_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Abs_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Abs_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Abs_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Abs_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Abs_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Abs_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_U_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_U_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_U_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_U_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_U_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_U_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_U_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_U_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_S_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_S_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_S_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_S_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_S_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_S_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Add_S_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_S_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_E_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_E_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_E_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_E_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_E_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_E_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_E_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Div_E_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Log_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Log_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Log_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Log_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Log_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Log_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Log_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Log_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sqrt_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sqrt_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sqrt_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sqrt_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sqrt_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sqrt_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sqrt_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sqrt_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Mul_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Mul_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Mul_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Mul_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Mul_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Mul_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Mul_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Mul_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Neg_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Neg_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Neg_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Neg_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Neg_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Neg_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Neg_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Neg_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Pow_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Pow_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Pow_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Pow_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Pow_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Pow_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Pow_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Pow_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_E_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_E_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_E_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_E_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_E_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_E_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_E_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Rem_E_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shl_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shl_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shl_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shl_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shl_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shl_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shl_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shl_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shr_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shr_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shr_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shr_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shr_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shr_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shr_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Shr_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sub_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sub_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sub_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sub_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sub_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sub_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sub_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sub_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sub_U_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_U_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sub_U_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_U_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sub_U_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_U_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_C_Sub_U_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_U_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Abs_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Abs_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Abs_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Abs_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Abs_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Abs_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Abs_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Abs_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_U_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_U_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_U_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_U_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_U_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_U_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_U_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_U_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_S_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_S_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_S_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_S_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_S_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_S_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Add_S_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_S_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_E_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_E_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_E_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_E_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_E_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_E_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_E_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Div_E_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Mul_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Mul_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Mul_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Mul_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Mul_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Mul_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Mul_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Mul_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Neg_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Neg_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Neg_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Neg_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Neg_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Neg_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Neg_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Neg_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Pow_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Pow_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Pow_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Pow_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Pow_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Pow_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Pow_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Pow_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_E_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_E_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_E_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_E_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_E_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_E_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_E_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Rem_E_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shl_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shl_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shl_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shl_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shl_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shl_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shl_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shl_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shr_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shr_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shr_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shr_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shr_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shr_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shr_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Shr_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Sub_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Sub_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Sub_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Sub_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Sub_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Sub_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Sub_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Sub_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Sub_U_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_U_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Sub_U_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_U_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Sub_U_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_U_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_O_Sub_U_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_U_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Abs_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Abs_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Abs_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Abs_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Abs_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Abs_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Abs_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Abs_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_U_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_U_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_U_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_U_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_U_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_U_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_U_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_U_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_S_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_S_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_S_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_S_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_S_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_S_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Add_S_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_S_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Div_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Div_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Div_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Div_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Div_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Div_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Div_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Div_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Mul_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Mul_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Mul_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Mul_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Mul_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Mul_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Mul_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Mul_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Neg_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Neg_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Neg_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Neg_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Neg_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Neg_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Neg_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Neg_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Pow_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Pow_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Pow_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Pow_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Pow_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Pow_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Pow_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Pow_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Sub_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Sub_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Sub_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Sub_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Sub_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Sub_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Sub_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Sub_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Sub_U_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_U_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Sub_U_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_U_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Sub_U_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_U_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_S_Sub_U_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_U_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Abs_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Abs_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Abs_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Abs_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Add_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Add_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Add_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Add_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Div_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Div_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Div_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Div_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Div_E_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Div_E_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Div_E_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Div_E_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Log_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Log_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Log_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Log_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Sqrt_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Sqrt_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Sqrt_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Sqrt_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Mul_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Mul_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Mul_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Mul_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Neg_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Neg_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Neg_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Neg_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Pow_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Pow_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Pow_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Pow_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rem_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rem_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rem_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rem_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rem_E_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rem_E_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Rem_E_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rem_E_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Cbrt_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Cbrt_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Cbrt_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Cbrt_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Sub_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Sub_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
fn parse_Sub_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Sub_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr);
}
