#![allow(non_snake_case)]
use crate::assembler::Assembler;
use crate::token::Token_Kind;
use byte_code::{Instruction, Register};
pub fn parse_instruction<'source>(asm: &mut Assembler<'source>) {
    if asm.matches(Token_Kind::Instruction) {
        let index = unsafe { asm.entry().value.integer };
        match index {
            0u64 => parse_halt(asm),
            1u64 => parse_trap(asm),
            2u64 => parse_call(asm),
            3u64 => parse_call_r(asm),
            4u64 => parse_call_i(asm),
            5u64 => parse_ret(asm),
            6u64 => parse_ecall(asm),
            7u64 => parse_jal(asm),
            8u64 => parse_jal_r(asm),
            9u64 => parse_jal_i(asm),
            10u64 => parse_jnz(asm),
            11u64 => parse_jnz_r(asm),
            12u64 => parse_jnz_i(asm),
            13u64 => parse_jiz(asm),
            14u64 => parse_jiz_r(asm),
            15u64 => parse_jiz_i(asm),
            16u64 => parse_ld_8(asm),
            17u64 => parse_ld_16(asm),
            18u64 => parse_ld_32(asm),
            19u64 => parse_ld_64(asm),
            20u64 => parse_ld_i(asm),
            21u64 => parse_ld_a_8(asm),
            22u64 => parse_ld_a_16(asm),
            23u64 => parse_ld_a_32(asm),
            24u64 => parse_ld_a_64(asm),
            25u64 => parse_st_8(asm),
            26u64 => parse_st_16(asm),
            27u64 => parse_st_32(asm),
            28u64 => parse_st_64(asm),
            29u64 => parse_st_i(asm),
            30u64 => parse_mov(asm),
            31u64 => parse_psh(asm),
            32u64 => parse_psh_i(asm),
            33u64 => parse_pop(asm),
            34u64 => parse_ie(asm),
            35u64 => parse_ie_f32(asm),
            36u64 => parse_ie_f64(asm),
            37u64 => parse_ne(asm),
            38u64 => parse_ne_f32(asm),
            39u64 => parse_ne_f64(asm),
            40u64 => parse_lt(asm),
            41u64 => parse_lt_f32(asm),
            42u64 => parse_lt_f64(asm),
            43u64 => parse_le(asm),
            44u64 => parse_le_f32(asm),
            45u64 => parse_le_f64(asm),
            46u64 => parse_gt(asm),
            47u64 => parse_gt_f32(asm),
            48u64 => parse_gt_f64(asm),
            49u64 => parse_ge(asm),
            50u64 => parse_ge_f32(asm),
            51u64 => parse_ge_f64(asm),
            52u64 => parse_and_i8(asm),
            53u64 => parse_and_i16(asm),
            54u64 => parse_and_i32(asm),
            55u64 => parse_and_i64(asm),
            56u64 => parse_and_u8(asm),
            57u64 => parse_and_u16(asm),
            58u64 => parse_and_u32(asm),
            59u64 => parse_and_u64(asm),
            60u64 => parse_or_i8(asm),
            61u64 => parse_or_i16(asm),
            62u64 => parse_or_i32(asm),
            63u64 => parse_or_i64(asm),
            64u64 => parse_or_u8(asm),
            65u64 => parse_or_u16(asm),
            66u64 => parse_or_u32(asm),
            67u64 => parse_or_u64(asm),
            68u64 => parse_xor_i8(asm),
            69u64 => parse_xor_i16(asm),
            70u64 => parse_xor_i32(asm),
            71u64 => parse_xor_i64(asm),
            72u64 => parse_xor_u8(asm),
            73u64 => parse_xor_u16(asm),
            74u64 => parse_xor_u32(asm),
            75u64 => parse_xor_u64(asm),
            76u64 => parse_not_i8(asm),
            77u64 => parse_not_i16(asm),
            78u64 => parse_not_i32(asm),
            79u64 => parse_not_i64(asm),
            80u64 => parse_not_u8(asm),
            81u64 => parse_not_u16(asm),
            82u64 => parse_not_u32(asm),
            83u64 => parse_not_u64(asm),
            84u64 => parse_shl_i8(asm),
            85u64 => parse_shl_i16(asm),
            86u64 => parse_shl_i32(asm),
            87u64 => parse_shl_i64(asm),
            88u64 => parse_shl_u8(asm),
            89u64 => parse_shl_u16(asm),
            90u64 => parse_shl_u32(asm),
            91u64 => parse_shl_u64(asm),
            92u64 => parse_shr_i8(asm),
            93u64 => parse_shr_i16(asm),
            94u64 => parse_shr_i32(asm),
            95u64 => parse_shr_i64(asm),
            96u64 => parse_shr_u8(asm),
            97u64 => parse_shr_u16(asm),
            98u64 => parse_shr_u32(asm),
            99u64 => parse_shr_u64(asm),
            100u64 => parse_rot_l_i8(asm),
            101u64 => parse_rot_l_i16(asm),
            102u64 => parse_rot_l_i32(asm),
            103u64 => parse_rot_l_i64(asm),
            104u64 => parse_rot_l_u8(asm),
            105u64 => parse_rot_l_u16(asm),
            106u64 => parse_rot_l_u32(asm),
            107u64 => parse_rot_l_u64(asm),
            108u64 => parse_rot_r_i8(asm),
            109u64 => parse_rot_r_i16(asm),
            110u64 => parse_rot_r_i32(asm),
            111u64 => parse_rot_r_i64(asm),
            112u64 => parse_rot_r_u8(asm),
            113u64 => parse_rot_r_u16(asm),
            114u64 => parse_rot_r_u32(asm),
            115u64 => parse_rot_r_u64(asm),
            116u64 => parse_c_ones_i8(asm),
            117u64 => parse_c_ones_i16(asm),
            118u64 => parse_c_ones_i32(asm),
            119u64 => parse_c_ones_i64(asm),
            120u64 => parse_c_ones_u8(asm),
            121u64 => parse_c_ones_u16(asm),
            122u64 => parse_c_ones_u32(asm),
            123u64 => parse_c_ones_u64(asm),
            124u64 => parse_l_ones_i8(asm),
            125u64 => parse_l_ones_i16(asm),
            126u64 => parse_l_ones_i32(asm),
            127u64 => parse_l_ones_i64(asm),
            128u64 => parse_l_ones_u8(asm),
            129u64 => parse_l_ones_u16(asm),
            130u64 => parse_l_ones_u32(asm),
            131u64 => parse_l_ones_u64(asm),
            132u64 => parse_t_ones_i8(asm),
            133u64 => parse_t_ones_i16(asm),
            134u64 => parse_t_ones_i32(asm),
            135u64 => parse_t_ones_i64(asm),
            136u64 => parse_t_ones_u8(asm),
            137u64 => parse_t_ones_u16(asm),
            138u64 => parse_t_ones_u32(asm),
            139u64 => parse_t_ones_u64(asm),
            140u64 => parse_c_zeros_i8(asm),
            141u64 => parse_c_zeros_i16(asm),
            142u64 => parse_c_zeros_i32(asm),
            143u64 => parse_c_zeros_i64(asm),
            144u64 => parse_c_zeros_u8(asm),
            145u64 => parse_c_zeros_u16(asm),
            146u64 => parse_c_zeros_u32(asm),
            147u64 => parse_c_zeros_u64(asm),
            148u64 => parse_l_zeros_i8(asm),
            149u64 => parse_l_zeros_i16(asm),
            150u64 => parse_l_zeros_i32(asm),
            151u64 => parse_l_zeros_i64(asm),
            152u64 => parse_l_zeros_u8(asm),
            153u64 => parse_l_zeros_u16(asm),
            154u64 => parse_l_zeros_u32(asm),
            155u64 => parse_l_zeros_u64(asm),
            156u64 => parse_t_zeros_i8(asm),
            157u64 => parse_t_zeros_i16(asm),
            158u64 => parse_t_zeros_i32(asm),
            159u64 => parse_t_zeros_i64(asm),
            160u64 => parse_t_zeros_u8(asm),
            161u64 => parse_t_zeros_u16(asm),
            162u64 => parse_t_zeros_u32(asm),
            163u64 => parse_t_zeros_u64(asm),
            164u64 => parse_r_bytes_i8(asm),
            165u64 => parse_r_bytes_i16(asm),
            166u64 => parse_r_bytes_i32(asm),
            167u64 => parse_r_bytes_i64(asm),
            168u64 => parse_r_bytes_u8(asm),
            169u64 => parse_r_bytes_u16(asm),
            170u64 => parse_r_bytes_u32(asm),
            171u64 => parse_r_bytes_u64(asm),
            172u64 => parse_r_bits_i8(asm),
            173u64 => parse_r_bits_i16(asm),
            174u64 => parse_r_bits_i32(asm),
            175u64 => parse_r_bits_i64(asm),
            176u64 => parse_r_bits_u8(asm),
            177u64 => parse_r_bits_u16(asm),
            178u64 => parse_r_bits_u32(asm),
            179u64 => parse_r_bits_u64(asm),
            180u64 => parse_c_abs_i8(asm),
            181u64 => parse_c_abs_i16(asm),
            182u64 => parse_c_abs_i32(asm),
            183u64 => parse_c_abs_i64(asm),
            184u64 => parse_c_add_i8(asm),
            185u64 => parse_c_add_i16(asm),
            186u64 => parse_c_add_i32(asm),
            187u64 => parse_c_add_i64(asm),
            188u64 => parse_c_add_u8(asm),
            189u64 => parse_c_add_u16(asm),
            190u64 => parse_c_add_u32(asm),
            191u64 => parse_c_add_u64(asm),
            192u64 => parse_c_add_u_i8(asm),
            193u64 => parse_c_add_u_i16(asm),
            194u64 => parse_c_add_u_i32(asm),
            195u64 => parse_c_add_u_i64(asm),
            196u64 => parse_c_add_s_u8(asm),
            197u64 => parse_c_add_s_u16(asm),
            198u64 => parse_c_add_s_u32(asm),
            199u64 => parse_c_add_s_u64(asm),
            200u64 => parse_c_div_i8(asm),
            201u64 => parse_c_div_i16(asm),
            202u64 => parse_c_div_i32(asm),
            203u64 => parse_c_div_i64(asm),
            204u64 => parse_c_div_u8(asm),
            205u64 => parse_c_div_u16(asm),
            206u64 => parse_c_div_u32(asm),
            207u64 => parse_c_div_u64(asm),
            208u64 => parse_c_div_e_i8(asm),
            209u64 => parse_c_div_e_i16(asm),
            210u64 => parse_c_div_e_i32(asm),
            211u64 => parse_c_div_e_i64(asm),
            212u64 => parse_c_div_e_u8(asm),
            213u64 => parse_c_div_e_u16(asm),
            214u64 => parse_c_div_e_u32(asm),
            215u64 => parse_c_div_e_u64(asm),
            216u64 => parse_c_log_i8(asm),
            217u64 => parse_c_log_i16(asm),
            218u64 => parse_c_log_i32(asm),
            219u64 => parse_c_log_i64(asm),
            220u64 => parse_c_log_u8(asm),
            221u64 => parse_c_log_u16(asm),
            222u64 => parse_c_log_u32(asm),
            223u64 => parse_c_log_u64(asm),
            224u64 => parse_c_sqrt_i8(asm),
            225u64 => parse_c_sqrt_i16(asm),
            226u64 => parse_c_sqrt_i32(asm),
            227u64 => parse_c_sqrt_i64(asm),
            228u64 => parse_c_sqrt_u8(asm),
            229u64 => parse_c_sqrt_u16(asm),
            230u64 => parse_c_sqrt_u32(asm),
            231u64 => parse_c_sqrt_u64(asm),
            232u64 => parse_c_mul_i8(asm),
            233u64 => parse_c_mul_i16(asm),
            234u64 => parse_c_mul_i32(asm),
            235u64 => parse_c_mul_i64(asm),
            236u64 => parse_c_mul_u8(asm),
            237u64 => parse_c_mul_u16(asm),
            238u64 => parse_c_mul_u32(asm),
            239u64 => parse_c_mul_u64(asm),
            240u64 => parse_c_neg_i8(asm),
            241u64 => parse_c_neg_i16(asm),
            242u64 => parse_c_neg_i32(asm),
            243u64 => parse_c_neg_i64(asm),
            244u64 => parse_c_pow_i8(asm),
            245u64 => parse_c_pow_i16(asm),
            246u64 => parse_c_pow_i32(asm),
            247u64 => parse_c_pow_i64(asm),
            248u64 => parse_c_pow_u8(asm),
            249u64 => parse_c_pow_u16(asm),
            250u64 => parse_c_pow_u32(asm),
            251u64 => parse_c_pow_u64(asm),
            252u64 => parse_c_rem_i8(asm),
            253u64 => parse_c_rem_i16(asm),
            254u64 => parse_c_rem_i32(asm),
            255u64 => parse_c_rem_i64(asm),
            256u64 => parse_c_rem_u8(asm),
            257u64 => parse_c_rem_u16(asm),
            258u64 => parse_c_rem_u32(asm),
            259u64 => parse_c_rem_u64(asm),
            260u64 => parse_c_rem_e_i8(asm),
            261u64 => parse_c_rem_e_i16(asm),
            262u64 => parse_c_rem_e_i32(asm),
            263u64 => parse_c_rem_e_i64(asm),
            264u64 => parse_c_rem_e_u8(asm),
            265u64 => parse_c_rem_e_u16(asm),
            266u64 => parse_c_rem_e_u32(asm),
            267u64 => parse_c_rem_e_u64(asm),
            268u64 => parse_c_shl_i8(asm),
            269u64 => parse_c_shl_i16(asm),
            270u64 => parse_c_shl_i32(asm),
            271u64 => parse_c_shl_i64(asm),
            272u64 => parse_c_shl_u8(asm),
            273u64 => parse_c_shl_u16(asm),
            274u64 => parse_c_shl_u32(asm),
            275u64 => parse_c_shl_u64(asm),
            276u64 => parse_c_shr_i8(asm),
            277u64 => parse_c_shr_i16(asm),
            278u64 => parse_c_shr_i32(asm),
            279u64 => parse_c_shr_i64(asm),
            280u64 => parse_c_shr_u8(asm),
            281u64 => parse_c_shr_u16(asm),
            282u64 => parse_c_shr_u32(asm),
            283u64 => parse_c_shr_u64(asm),
            284u64 => parse_c_sub_i8(asm),
            285u64 => parse_c_sub_i16(asm),
            286u64 => parse_c_sub_i32(asm),
            287u64 => parse_c_sub_i64(asm),
            288u64 => parse_c_sub_u8(asm),
            289u64 => parse_c_sub_u16(asm),
            290u64 => parse_c_sub_u32(asm),
            291u64 => parse_c_sub_u64(asm),
            292u64 => parse_c_sub_u_i8(asm),
            293u64 => parse_c_sub_u_i16(asm),
            294u64 => parse_c_sub_u_i32(asm),
            295u64 => parse_c_sub_u_i64(asm),
            296u64 => parse_o_abs_i8(asm),
            297u64 => parse_o_abs_i16(asm),
            298u64 => parse_o_abs_i32(asm),
            299u64 => parse_o_abs_i64(asm),
            300u64 => parse_o_add_i8(asm),
            301u64 => parse_o_add_i16(asm),
            302u64 => parse_o_add_i32(asm),
            303u64 => parse_o_add_i64(asm),
            304u64 => parse_o_add_u8(asm),
            305u64 => parse_o_add_u16(asm),
            306u64 => parse_o_add_u32(asm),
            307u64 => parse_o_add_u64(asm),
            308u64 => parse_o_add_u_i8(asm),
            309u64 => parse_o_add_u_i16(asm),
            310u64 => parse_o_add_u_i32(asm),
            311u64 => parse_o_add_u_i64(asm),
            312u64 => parse_o_add_s_u8(asm),
            313u64 => parse_o_add_s_u16(asm),
            314u64 => parse_o_add_s_u32(asm),
            315u64 => parse_o_add_s_u64(asm),
            316u64 => parse_o_div_i8(asm),
            317u64 => parse_o_div_i16(asm),
            318u64 => parse_o_div_i32(asm),
            319u64 => parse_o_div_i64(asm),
            320u64 => parse_o_div_u8(asm),
            321u64 => parse_o_div_u16(asm),
            322u64 => parse_o_div_u32(asm),
            323u64 => parse_o_div_u64(asm),
            324u64 => parse_o_div_e_i8(asm),
            325u64 => parse_o_div_e_i16(asm),
            326u64 => parse_o_div_e_i32(asm),
            327u64 => parse_o_div_e_i64(asm),
            328u64 => parse_o_div_e_u8(asm),
            329u64 => parse_o_div_e_u16(asm),
            330u64 => parse_o_div_e_u32(asm),
            331u64 => parse_o_div_e_u64(asm),
            332u64 => parse_o_mul_i8(asm),
            333u64 => parse_o_mul_i16(asm),
            334u64 => parse_o_mul_i32(asm),
            335u64 => parse_o_mul_i64(asm),
            336u64 => parse_o_mul_u8(asm),
            337u64 => parse_o_mul_u16(asm),
            338u64 => parse_o_mul_u32(asm),
            339u64 => parse_o_mul_u64(asm),
            340u64 => parse_o_neg_i8(asm),
            341u64 => parse_o_neg_i16(asm),
            342u64 => parse_o_neg_i32(asm),
            343u64 => parse_o_neg_i64(asm),
            344u64 => parse_o_pow_i8(asm),
            345u64 => parse_o_pow_i16(asm),
            346u64 => parse_o_pow_i32(asm),
            347u64 => parse_o_pow_i64(asm),
            348u64 => parse_o_pow_u8(asm),
            349u64 => parse_o_pow_u16(asm),
            350u64 => parse_o_pow_u32(asm),
            351u64 => parse_o_pow_u64(asm),
            352u64 => parse_o_rem_i8(asm),
            353u64 => parse_o_rem_i16(asm),
            354u64 => parse_o_rem_i32(asm),
            355u64 => parse_o_rem_i64(asm),
            356u64 => parse_o_rem_u8(asm),
            357u64 => parse_o_rem_u16(asm),
            358u64 => parse_o_rem_u32(asm),
            359u64 => parse_o_rem_u64(asm),
            360u64 => parse_o_rem_e_i8(asm),
            361u64 => parse_o_rem_e_i16(asm),
            362u64 => parse_o_rem_e_i32(asm),
            363u64 => parse_o_rem_e_i64(asm),
            364u64 => parse_o_rem_e_u8(asm),
            365u64 => parse_o_rem_e_u16(asm),
            366u64 => parse_o_rem_e_u32(asm),
            367u64 => parse_o_rem_e_u64(asm),
            368u64 => parse_o_shl_i8(asm),
            369u64 => parse_o_shl_i16(asm),
            370u64 => parse_o_shl_i32(asm),
            371u64 => parse_o_shl_i64(asm),
            372u64 => parse_o_shl_u8(asm),
            373u64 => parse_o_shl_u16(asm),
            374u64 => parse_o_shl_u32(asm),
            375u64 => parse_o_shl_u64(asm),
            376u64 => parse_o_shr_i8(asm),
            377u64 => parse_o_shr_i16(asm),
            378u64 => parse_o_shr_i32(asm),
            379u64 => parse_o_shr_i64(asm),
            380u64 => parse_o_shr_u8(asm),
            381u64 => parse_o_shr_u16(asm),
            382u64 => parse_o_shr_u32(asm),
            383u64 => parse_o_shr_u64(asm),
            384u64 => parse_o_sub_i8(asm),
            385u64 => parse_o_sub_i16(asm),
            386u64 => parse_o_sub_i32(asm),
            387u64 => parse_o_sub_i64(asm),
            388u64 => parse_o_sub_u8(asm),
            389u64 => parse_o_sub_u16(asm),
            390u64 => parse_o_sub_u32(asm),
            391u64 => parse_o_sub_u64(asm),
            392u64 => parse_o_sub_u_i8(asm),
            393u64 => parse_o_sub_u_i16(asm),
            394u64 => parse_o_sub_u_i32(asm),
            395u64 => parse_o_sub_u_i64(asm),
            396u64 => parse_s_abs_i8(asm),
            397u64 => parse_s_abs_i16(asm),
            398u64 => parse_s_abs_i32(asm),
            399u64 => parse_s_abs_i64(asm),
            400u64 => parse_s_add_i8(asm),
            401u64 => parse_s_add_i16(asm),
            402u64 => parse_s_add_i32(asm),
            403u64 => parse_s_add_i64(asm),
            404u64 => parse_s_add_u8(asm),
            405u64 => parse_s_add_u16(asm),
            406u64 => parse_s_add_u32(asm),
            407u64 => parse_s_add_u64(asm),
            408u64 => parse_s_add_u_i8(asm),
            409u64 => parse_s_add_u_i16(asm),
            410u64 => parse_s_add_u_i32(asm),
            411u64 => parse_s_add_u_i64(asm),
            412u64 => parse_s_add_s_u8(asm),
            413u64 => parse_s_add_s_u16(asm),
            414u64 => parse_s_add_s_u32(asm),
            415u64 => parse_s_add_s_u64(asm),
            416u64 => parse_s_div_i8(asm),
            417u64 => parse_s_div_i16(asm),
            418u64 => parse_s_div_i32(asm),
            419u64 => parse_s_div_i64(asm),
            420u64 => parse_s_div_u8(asm),
            421u64 => parse_s_div_u16(asm),
            422u64 => parse_s_div_u32(asm),
            423u64 => parse_s_div_u64(asm),
            424u64 => parse_s_mul_i8(asm),
            425u64 => parse_s_mul_i16(asm),
            426u64 => parse_s_mul_i32(asm),
            427u64 => parse_s_mul_i64(asm),
            428u64 => parse_s_mul_u8(asm),
            429u64 => parse_s_mul_u16(asm),
            430u64 => parse_s_mul_u32(asm),
            431u64 => parse_s_mul_u64(asm),
            432u64 => parse_s_neg_i8(asm),
            433u64 => parse_s_neg_i16(asm),
            434u64 => parse_s_neg_i32(asm),
            435u64 => parse_s_neg_i64(asm),
            436u64 => parse_s_pow_i8(asm),
            437u64 => parse_s_pow_i16(asm),
            438u64 => parse_s_pow_i32(asm),
            439u64 => parse_s_pow_i64(asm),
            440u64 => parse_s_pow_u8(asm),
            441u64 => parse_s_pow_u16(asm),
            442u64 => parse_s_pow_u32(asm),
            443u64 => parse_s_pow_u64(asm),
            444u64 => parse_s_sub_i8(asm),
            445u64 => parse_s_sub_i16(asm),
            446u64 => parse_s_sub_i32(asm),
            447u64 => parse_s_sub_i64(asm),
            448u64 => parse_s_sub_u8(asm),
            449u64 => parse_s_sub_u16(asm),
            450u64 => parse_s_sub_u32(asm),
            451u64 => parse_s_sub_u64(asm),
            452u64 => parse_s_sub_u_i8(asm),
            453u64 => parse_s_sub_u_i16(asm),
            454u64 => parse_s_sub_u_i32(asm),
            455u64 => parse_s_sub_u_i64(asm),
            456u64 => parse_abs_f32(asm),
            457u64 => parse_abs_f64(asm),
            458u64 => parse_add_f32(asm),
            459u64 => parse_add_f64(asm),
            460u64 => parse_div_f32(asm),
            461u64 => parse_div_f64(asm),
            462u64 => parse_div_e_f32(asm),
            463u64 => parse_div_e_f64(asm),
            464u64 => parse_log_f32(asm),
            465u64 => parse_log_f64(asm),
            466u64 => parse_sqrt_f32(asm),
            467u64 => parse_sqrt_f64(asm),
            468u64 => parse_mul_f32(asm),
            469u64 => parse_mul_f64(asm),
            470u64 => parse_neg_f32(asm),
            471u64 => parse_neg_f64(asm),
            472u64 => parse_pow_f32(asm),
            473u64 => parse_pow_f64(asm),
            474u64 => parse_rem_f32(asm),
            475u64 => parse_rem_f64(asm),
            476u64 => parse_rem_e_f32(asm),
            477u64 => parse_rem_e_f64(asm),
            478u64 => parse_cbrt_f32(asm),
            479u64 => parse_cbrt_f64(asm),
            480u64 => parse_sub_f32(asm),
            481u64 => parse_sub_f64(asm),
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
fn parse_imm<'source>(asm: &mut Assembler<'source>) -> i16 {
    if asm.matches(Token_Kind::Colon) {
        asm.expects(Token_Kind::Identifier);
        asm.patch_label()
    } else if asm.matches(Token_Kind::Ampersand) {
        asm.expects(Token_Kind::Identifier);
        asm.patch_address()
    } else {
        asm.expects(Token_Kind::Number);
        unsafe { asm.entry().value.integer as i16 }
    }
}
fn parse_halt<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Halt;
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_trap<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Trap;
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_call<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Call {
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_call_r<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Call_R {
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_call_i<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Call_I {
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ret<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ret;
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ecall<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ecall {
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_jal<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jal {
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_jal_r<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jal_R {
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_jal_i<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jal_I {
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_jnz<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jnz {
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_jnz_r<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jnz_R {
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_jnz_i<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jnz_I {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_jiz<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jiz {
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_jiz_r<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jiz_R {
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_jiz_i<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Jiz_I {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ld_8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ld_8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ld_16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ld_16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ld_32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ld_32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ld_64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ld_64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ld_i<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ld_I {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ld_a_8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ld_A_8 {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm) as u16,
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ld_a_16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ld_A_16 {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm) as u16,
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ld_a_32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ld_A_32 {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm) as u16,
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ld_a_64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ld_A_64 {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm) as u16,
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_st_8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::St_8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_st_16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::St_16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_st_32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::St_32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_st_64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::St_64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_st_i<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::St_I {
        rd: parse_register_comma(asm),
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_mov<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Mov {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_psh<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Psh {
        rd: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_psh_i<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Psh_I {
        imm: parse_imm(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_pop<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Pop {
        rd: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ie<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ie {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ie_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ie_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ie_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ie_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ne<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ne {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ne_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ne_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ne_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ne_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_lt<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Lt {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_lt_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Lt_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_lt_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Lt_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_le<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Le {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_le_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Le_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_le_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Le_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_gt<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Gt {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_gt_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Gt_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_gt_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Gt_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ge<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ge {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ge_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ge_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_ge_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Ge_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_and_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_and_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_and_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_and_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_and_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_and_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_and_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_and_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::And_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_or_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_or_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_or_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_or_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_or_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_or_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_or_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_or_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Or_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_xor_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_xor_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_xor_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_xor_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_xor_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_xor_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_xor_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_xor_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Xor_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_not_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_not_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_not_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_not_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_not_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_not_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_not_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_not_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Not_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shl_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shl_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shl_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shl_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shl_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shl_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shl_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shl_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shl_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shr_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shr_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shr_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shr_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shr_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shr_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shr_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_shr_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Shr_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_l_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_l_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_l_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_l_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_l_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_l_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_l_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_l_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_L_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_r_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_r_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_r_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_r_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_r_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_r_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_r_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rot_r_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rot_R_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_ones_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_ones_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_ones_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_ones_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_ones_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_ones_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_ones_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_ones_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Ones_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_ones_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_ones_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_ones_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_ones_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_ones_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_ones_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_ones_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_ones_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Ones_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_ones_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_ones_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_ones_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_ones_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_ones_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_ones_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_ones_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_ones_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Ones_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_zeros_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_zeros_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_zeros_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_zeros_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_zeros_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_zeros_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_zeros_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_zeros_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Zeros_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_zeros_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_zeros_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_zeros_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_zeros_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_zeros_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_zeros_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_zeros_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_l_zeros_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::L_Zeros_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_zeros_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_zeros_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_zeros_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_zeros_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_zeros_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_zeros_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_zeros_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_t_zeros_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::T_Zeros_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bytes_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bytes_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bytes_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bytes_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bytes_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bytes_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bytes_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bytes_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bytes_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bits_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bits_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bits_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bits_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bits_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bits_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bits_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_r_bits_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::R_Bits_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_abs_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Abs_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_abs_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Abs_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_abs_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Abs_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_abs_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Abs_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_u_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_U_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_u_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_U_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_u_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_U_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_u_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_U_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_s_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_S_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_s_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_S_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_s_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_S_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_add_s_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Add_S_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_e_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_e_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_e_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_e_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_e_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_e_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_e_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_div_e_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Div_E_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_log_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_log_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_log_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_log_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_log_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_log_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_log_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_log_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Log_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sqrt_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sqrt_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sqrt_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sqrt_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sqrt_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sqrt_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sqrt_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sqrt_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sqrt_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_mul_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_mul_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_mul_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_mul_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_mul_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_mul_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_mul_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_mul_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Mul_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_neg_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Neg_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_neg_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Neg_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_neg_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Neg_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_neg_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Neg_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_pow_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_pow_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_pow_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_pow_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_pow_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_pow_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_pow_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_pow_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Pow_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_e_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_e_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_e_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_e_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_e_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_e_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_e_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_rem_e_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Rem_E_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shl_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shl_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shl_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shl_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shl_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shl_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shl_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shl_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shl_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shr_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shr_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shr_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shr_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shr_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shr_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shr_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_shr_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Shr_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sub_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sub_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sub_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sub_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sub_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sub_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sub_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sub_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sub_u_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_U_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sub_u_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_U_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sub_u_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_U_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_c_sub_u_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::C_Sub_U_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_abs_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Abs_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_abs_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Abs_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_abs_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Abs_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_abs_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Abs_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_u_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_U_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_u_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_U_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_u_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_U_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_u_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_U_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_s_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_S_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_s_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_S_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_s_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_S_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_add_s_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Add_S_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_e_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_e_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_e_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_e_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_e_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_e_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_e_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_div_e_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Div_E_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_mul_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_mul_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_mul_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_mul_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_mul_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_mul_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_mul_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_mul_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Mul_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_neg_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Neg_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_neg_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Neg_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_neg_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Neg_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_neg_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Neg_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_pow_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_pow_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_pow_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_pow_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_pow_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_pow_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_pow_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_pow_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Pow_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_e_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_e_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_e_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_e_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_e_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_e_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_e_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_rem_e_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Rem_E_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shl_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shl_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shl_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shl_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shl_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shl_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shl_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shl_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shl_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shr_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shr_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shr_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shr_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shr_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shr_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shr_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_shr_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Shr_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_sub_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_sub_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_sub_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_sub_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_sub_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_sub_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_sub_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_sub_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_sub_u_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_U_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_sub_u_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_U_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_sub_u_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_U_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_o_sub_u_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::O_Sub_U_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_abs_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Abs_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_abs_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Abs_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_abs_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Abs_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_abs_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Abs_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_u_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_U_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_u_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_U_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_u_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_U_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_u_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_U_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_s_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_S_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_s_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_S_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_s_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_S_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_add_s_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Add_S_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_div_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_div_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_div_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_div_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_div_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_div_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_div_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_div_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Div_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_mul_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_mul_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_mul_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_mul_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_mul_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_mul_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_mul_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_mul_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Mul_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_neg_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Neg_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_neg_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Neg_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_neg_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Neg_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_neg_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Neg_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_pow_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_pow_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_pow_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_pow_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_pow_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_pow_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_pow_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_pow_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Pow_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_sub_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_sub_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_sub_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_sub_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_sub_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_u8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_sub_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_u16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_sub_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_u32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_sub_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_u64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_sub_u_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_U_i8 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_sub_u_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_U_i16 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_sub_u_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_U_i32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_s_sub_u_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::S_Sub_U_i64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_abs_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Abs_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_abs_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Abs_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_add_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Add_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_add_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Add_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_div_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Div_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_div_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Div_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_div_e_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Div_E_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_div_e_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Div_E_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_log_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Log_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_log_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Log_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_sqrt_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Sqrt_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_sqrt_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Sqrt_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_mul_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Mul_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_mul_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Mul_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_neg_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Neg_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_neg_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Neg_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_pow_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Pow_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_pow_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Pow_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rem_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rem_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rem_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rem_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rem_e_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rem_E_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_rem_e_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Rem_E_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_cbrt_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Cbrt_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_cbrt_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Cbrt_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_sub_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Sub_f32 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
fn parse_sub_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = Instruction::Sub_f64 {
        rd: parse_register_comma(asm),
        rs1: parse_register_comma(asm),
        rs2: parse_register(asm),
    };
    asm.expects(Token_Kind::Newline);
    asm.object.code_instrs.push(instr.encode());
}
