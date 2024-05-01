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
            3u64 => parse_ret(asm),
            4u64 => parse_ecall(asm),
            5u64 => parse_jal(asm),
            6u64 => parse_bie(asm),
            7u64 => parse_bne(asm),
            8u64 => parse_blts(asm),
            9u64 => parse_bltu(asm),
            10u64 => parse_bles(asm),
            11u64 => parse_bleu(asm),
            12u64 => parse_bgts(asm),
            13u64 => parse_bgtu(asm),
            14u64 => parse_bges(asm),
            15u64 => parse_bgeu(asm),
            16u64 => parse_bie_f32(asm),
            17u64 => parse_bie_f64(asm),
            18u64 => parse_bne_f32(asm),
            19u64 => parse_bne_f64(asm),
            20u64 => parse_blt_f32(asm),
            21u64 => parse_blt_f64(asm),
            22u64 => parse_ble_f32(asm),
            23u64 => parse_ble_f64(asm),
            24u64 => parse_bgt_f32(asm),
            25u64 => parse_bgt_f64(asm),
            26u64 => parse_bge_f32(asm),
            27u64 => parse_bge_f64(asm),
            28u64 => parse_cie(asm),
            29u64 => parse_cie_f32(asm),
            30u64 => parse_cie_f64(asm),
            31u64 => parse_cne(asm),
            32u64 => parse_cne_f32(asm),
            33u64 => parse_cne_f64(asm),
            34u64 => parse_clts(asm),
            35u64 => parse_cltu(asm),
            36u64 => parse_clt_f32(asm),
            37u64 => parse_clt_f64(asm),
            38u64 => parse_cles(asm),
            39u64 => parse_cleu(asm),
            40u64 => parse_cle_f32(asm),
            41u64 => parse_cle_f64(asm),
            42u64 => parse_cgts(asm),
            43u64 => parse_cgtu(asm),
            44u64 => parse_cgt_f32(asm),
            45u64 => parse_cgt_f64(asm),
            46u64 => parse_cges(asm),
            47u64 => parse_cgeu(asm),
            48u64 => parse_cge_f32(asm),
            49u64 => parse_cge_f64(asm),
            50u64 => parse_lra_8(asm),
            51u64 => parse_lra_16(asm),
            52u64 => parse_lra_32(asm),
            53u64 => parse_lra_64(asm),
            54u64 => parse_lsi(asm),
            55u64 => parse_lui(asm),
            56u64 => parse_lia_8(asm),
            57u64 => parse_lia_16(asm),
            58u64 => parse_lia_32(asm),
            59u64 => parse_lia_64(asm),
            60u64 => parse_sra_8(asm),
            61u64 => parse_sra_16(asm),
            62u64 => parse_sra_32(asm),
            63u64 => parse_sra_64(asm),
            64u64 => parse_ssi(asm),
            65u64 => parse_sui(asm),
            66u64 => parse_mov(asm),
            67u64 => parse_pop(asm),
            68u64 => parse_psh(asm),
            69u64 => parse_psi(asm),
            70u64 => parse_pui(asm),
            71u64 => parse_and_i8(asm),
            72u64 => parse_and_i16(asm),
            73u64 => parse_and_i32(asm),
            74u64 => parse_and_i64(asm),
            75u64 => parse_and_u8(asm),
            76u64 => parse_and_u16(asm),
            77u64 => parse_and_u32(asm),
            78u64 => parse_and_u64(asm),
            79u64 => parse_or_i8(asm),
            80u64 => parse_or_i16(asm),
            81u64 => parse_or_i32(asm),
            82u64 => parse_or_i64(asm),
            83u64 => parse_or_u8(asm),
            84u64 => parse_or_u16(asm),
            85u64 => parse_or_u32(asm),
            86u64 => parse_or_u64(asm),
            87u64 => parse_xor_i8(asm),
            88u64 => parse_xor_i16(asm),
            89u64 => parse_xor_i32(asm),
            90u64 => parse_xor_i64(asm),
            91u64 => parse_xor_u8(asm),
            92u64 => parse_xor_u16(asm),
            93u64 => parse_xor_u32(asm),
            94u64 => parse_xor_u64(asm),
            95u64 => parse_not_i8(asm),
            96u64 => parse_not_i16(asm),
            97u64 => parse_not_i32(asm),
            98u64 => parse_not_i64(asm),
            99u64 => parse_not_u8(asm),
            100u64 => parse_not_u16(asm),
            101u64 => parse_not_u32(asm),
            102u64 => parse_not_u64(asm),
            103u64 => parse_shl_i8(asm),
            104u64 => parse_shl_i16(asm),
            105u64 => parse_shl_i32(asm),
            106u64 => parse_shl_i64(asm),
            107u64 => parse_shl_u8(asm),
            108u64 => parse_shl_u16(asm),
            109u64 => parse_shl_u32(asm),
            110u64 => parse_shl_u64(asm),
            111u64 => parse_shr_i8(asm),
            112u64 => parse_shr_i16(asm),
            113u64 => parse_shr_i32(asm),
            114u64 => parse_shr_i64(asm),
            115u64 => parse_shr_u8(asm),
            116u64 => parse_shr_u16(asm),
            117u64 => parse_shr_u32(asm),
            118u64 => parse_shr_u64(asm),
            119u64 => parse_rot_l_i8(asm),
            120u64 => parse_rot_l_i16(asm),
            121u64 => parse_rot_l_i32(asm),
            122u64 => parse_rot_l_i64(asm),
            123u64 => parse_rot_l_u8(asm),
            124u64 => parse_rot_l_u16(asm),
            125u64 => parse_rot_l_u32(asm),
            126u64 => parse_rot_l_u64(asm),
            127u64 => parse_rot_r_i8(asm),
            128u64 => parse_rot_r_i16(asm),
            129u64 => parse_rot_r_i32(asm),
            130u64 => parse_rot_r_i64(asm),
            131u64 => parse_rot_r_u8(asm),
            132u64 => parse_rot_r_u16(asm),
            133u64 => parse_rot_r_u32(asm),
            134u64 => parse_rot_r_u64(asm),
            135u64 => parse_c_ones_i8(asm),
            136u64 => parse_c_ones_i16(asm),
            137u64 => parse_c_ones_i32(asm),
            138u64 => parse_c_ones_i64(asm),
            139u64 => parse_c_ones_u8(asm),
            140u64 => parse_c_ones_u16(asm),
            141u64 => parse_c_ones_u32(asm),
            142u64 => parse_c_ones_u64(asm),
            143u64 => parse_l_ones_i8(asm),
            144u64 => parse_l_ones_i16(asm),
            145u64 => parse_l_ones_i32(asm),
            146u64 => parse_l_ones_i64(asm),
            147u64 => parse_l_ones_u8(asm),
            148u64 => parse_l_ones_u16(asm),
            149u64 => parse_l_ones_u32(asm),
            150u64 => parse_l_ones_u64(asm),
            151u64 => parse_t_ones_i8(asm),
            152u64 => parse_t_ones_i16(asm),
            153u64 => parse_t_ones_i32(asm),
            154u64 => parse_t_ones_i64(asm),
            155u64 => parse_t_ones_u8(asm),
            156u64 => parse_t_ones_u16(asm),
            157u64 => parse_t_ones_u32(asm),
            158u64 => parse_t_ones_u64(asm),
            159u64 => parse_c_zeros_i8(asm),
            160u64 => parse_c_zeros_i16(asm),
            161u64 => parse_c_zeros_i32(asm),
            162u64 => parse_c_zeros_i64(asm),
            163u64 => parse_c_zeros_u8(asm),
            164u64 => parse_c_zeros_u16(asm),
            165u64 => parse_c_zeros_u32(asm),
            166u64 => parse_c_zeros_u64(asm),
            167u64 => parse_l_zeros_i8(asm),
            168u64 => parse_l_zeros_i16(asm),
            169u64 => parse_l_zeros_i32(asm),
            170u64 => parse_l_zeros_i64(asm),
            171u64 => parse_l_zeros_u8(asm),
            172u64 => parse_l_zeros_u16(asm),
            173u64 => parse_l_zeros_u32(asm),
            174u64 => parse_l_zeros_u64(asm),
            175u64 => parse_t_zeros_i8(asm),
            176u64 => parse_t_zeros_i16(asm),
            177u64 => parse_t_zeros_i32(asm),
            178u64 => parse_t_zeros_i64(asm),
            179u64 => parse_t_zeros_u8(asm),
            180u64 => parse_t_zeros_u16(asm),
            181u64 => parse_t_zeros_u32(asm),
            182u64 => parse_t_zeros_u64(asm),
            183u64 => parse_r_bytes_i8(asm),
            184u64 => parse_r_bytes_i16(asm),
            185u64 => parse_r_bytes_i32(asm),
            186u64 => parse_r_bytes_i64(asm),
            187u64 => parse_r_bytes_u8(asm),
            188u64 => parse_r_bytes_u16(asm),
            189u64 => parse_r_bytes_u32(asm),
            190u64 => parse_r_bytes_u64(asm),
            191u64 => parse_r_bits_i8(asm),
            192u64 => parse_r_bits_i16(asm),
            193u64 => parse_r_bits_i32(asm),
            194u64 => parse_r_bits_i64(asm),
            195u64 => parse_r_bits_u8(asm),
            196u64 => parse_r_bits_u16(asm),
            197u64 => parse_r_bits_u32(asm),
            198u64 => parse_r_bits_u64(asm),
            199u64 => parse_c_abs_i8(asm),
            200u64 => parse_c_abs_i16(asm),
            201u64 => parse_c_abs_i32(asm),
            202u64 => parse_c_abs_i64(asm),
            203u64 => parse_c_add_i8(asm),
            204u64 => parse_c_add_i16(asm),
            205u64 => parse_c_add_i32(asm),
            206u64 => parse_c_add_i64(asm),
            207u64 => parse_c_add_u8(asm),
            208u64 => parse_c_add_u16(asm),
            209u64 => parse_c_add_u32(asm),
            210u64 => parse_c_add_u64(asm),
            211u64 => parse_c_add_u_i8(asm),
            212u64 => parse_c_add_u_i16(asm),
            213u64 => parse_c_add_u_i32(asm),
            214u64 => parse_c_add_u_i64(asm),
            215u64 => parse_c_add_s_u8(asm),
            216u64 => parse_c_add_s_u16(asm),
            217u64 => parse_c_add_s_u32(asm),
            218u64 => parse_c_add_s_u64(asm),
            219u64 => parse_c_div_i8(asm),
            220u64 => parse_c_div_i16(asm),
            221u64 => parse_c_div_i32(asm),
            222u64 => parse_c_div_i64(asm),
            223u64 => parse_c_div_u8(asm),
            224u64 => parse_c_div_u16(asm),
            225u64 => parse_c_div_u32(asm),
            226u64 => parse_c_div_u64(asm),
            227u64 => parse_c_div_e_i8(asm),
            228u64 => parse_c_div_e_i16(asm),
            229u64 => parse_c_div_e_i32(asm),
            230u64 => parse_c_div_e_i64(asm),
            231u64 => parse_c_div_e_u8(asm),
            232u64 => parse_c_div_e_u16(asm),
            233u64 => parse_c_div_e_u32(asm),
            234u64 => parse_c_div_e_u64(asm),
            235u64 => parse_c_log_i8(asm),
            236u64 => parse_c_log_i16(asm),
            237u64 => parse_c_log_i32(asm),
            238u64 => parse_c_log_i64(asm),
            239u64 => parse_c_log_u8(asm),
            240u64 => parse_c_log_u16(asm),
            241u64 => parse_c_log_u32(asm),
            242u64 => parse_c_log_u64(asm),
            243u64 => parse_c_sqrt_i8(asm),
            244u64 => parse_c_sqrt_i16(asm),
            245u64 => parse_c_sqrt_i32(asm),
            246u64 => parse_c_sqrt_i64(asm),
            247u64 => parse_c_sqrt_u8(asm),
            248u64 => parse_c_sqrt_u16(asm),
            249u64 => parse_c_sqrt_u32(asm),
            250u64 => parse_c_sqrt_u64(asm),
            251u64 => parse_c_mul_i8(asm),
            252u64 => parse_c_mul_i16(asm),
            253u64 => parse_c_mul_i32(asm),
            254u64 => parse_c_mul_i64(asm),
            255u64 => parse_c_mul_u8(asm),
            256u64 => parse_c_mul_u16(asm),
            257u64 => parse_c_mul_u32(asm),
            258u64 => parse_c_mul_u64(asm),
            259u64 => parse_c_neg_i8(asm),
            260u64 => parse_c_neg_i16(asm),
            261u64 => parse_c_neg_i32(asm),
            262u64 => parse_c_neg_i64(asm),
            263u64 => parse_c_pow_i8(asm),
            264u64 => parse_c_pow_i16(asm),
            265u64 => parse_c_pow_i32(asm),
            266u64 => parse_c_pow_i64(asm),
            267u64 => parse_c_pow_u8(asm),
            268u64 => parse_c_pow_u16(asm),
            269u64 => parse_c_pow_u32(asm),
            270u64 => parse_c_pow_u64(asm),
            271u64 => parse_c_rem_i8(asm),
            272u64 => parse_c_rem_i16(asm),
            273u64 => parse_c_rem_i32(asm),
            274u64 => parse_c_rem_i64(asm),
            275u64 => parse_c_rem_u8(asm),
            276u64 => parse_c_rem_u16(asm),
            277u64 => parse_c_rem_u32(asm),
            278u64 => parse_c_rem_u64(asm),
            279u64 => parse_c_rem_e_i8(asm),
            280u64 => parse_c_rem_e_i16(asm),
            281u64 => parse_c_rem_e_i32(asm),
            282u64 => parse_c_rem_e_i64(asm),
            283u64 => parse_c_rem_e_u8(asm),
            284u64 => parse_c_rem_e_u16(asm),
            285u64 => parse_c_rem_e_u32(asm),
            286u64 => parse_c_rem_e_u64(asm),
            287u64 => parse_c_shl_i8(asm),
            288u64 => parse_c_shl_i16(asm),
            289u64 => parse_c_shl_i32(asm),
            290u64 => parse_c_shl_i64(asm),
            291u64 => parse_c_shl_u8(asm),
            292u64 => parse_c_shl_u16(asm),
            293u64 => parse_c_shl_u32(asm),
            294u64 => parse_c_shl_u64(asm),
            295u64 => parse_c_shr_i8(asm),
            296u64 => parse_c_shr_i16(asm),
            297u64 => parse_c_shr_i32(asm),
            298u64 => parse_c_shr_i64(asm),
            299u64 => parse_c_shr_u8(asm),
            300u64 => parse_c_shr_u16(asm),
            301u64 => parse_c_shr_u32(asm),
            302u64 => parse_c_shr_u64(asm),
            303u64 => parse_c_sub_i8(asm),
            304u64 => parse_c_sub_i16(asm),
            305u64 => parse_c_sub_i32(asm),
            306u64 => parse_c_sub_i64(asm),
            307u64 => parse_c_sub_u8(asm),
            308u64 => parse_c_sub_u16(asm),
            309u64 => parse_c_sub_u32(asm),
            310u64 => parse_c_sub_u64(asm),
            311u64 => parse_c_sub_u_i8(asm),
            312u64 => parse_c_sub_u_i16(asm),
            313u64 => parse_c_sub_u_i32(asm),
            314u64 => parse_c_sub_u_i64(asm),
            315u64 => parse_o_abs_i8(asm),
            316u64 => parse_o_abs_i16(asm),
            317u64 => parse_o_abs_i32(asm),
            318u64 => parse_o_abs_i64(asm),
            319u64 => parse_o_add_i8(asm),
            320u64 => parse_o_add_i16(asm),
            321u64 => parse_o_add_i32(asm),
            322u64 => parse_o_add_i64(asm),
            323u64 => parse_o_add_u8(asm),
            324u64 => parse_o_add_u16(asm),
            325u64 => parse_o_add_u32(asm),
            326u64 => parse_o_add_u64(asm),
            327u64 => parse_o_add_u_i8(asm),
            328u64 => parse_o_add_u_i16(asm),
            329u64 => parse_o_add_u_i32(asm),
            330u64 => parse_o_add_u_i64(asm),
            331u64 => parse_o_add_s_u8(asm),
            332u64 => parse_o_add_s_u16(asm),
            333u64 => parse_o_add_s_u32(asm),
            334u64 => parse_o_add_s_u64(asm),
            335u64 => parse_o_div_i8(asm),
            336u64 => parse_o_div_i16(asm),
            337u64 => parse_o_div_i32(asm),
            338u64 => parse_o_div_i64(asm),
            339u64 => parse_o_div_u8(asm),
            340u64 => parse_o_div_u16(asm),
            341u64 => parse_o_div_u32(asm),
            342u64 => parse_o_div_u64(asm),
            343u64 => parse_o_div_e_i8(asm),
            344u64 => parse_o_div_e_i16(asm),
            345u64 => parse_o_div_e_i32(asm),
            346u64 => parse_o_div_e_i64(asm),
            347u64 => parse_o_div_e_u8(asm),
            348u64 => parse_o_div_e_u16(asm),
            349u64 => parse_o_div_e_u32(asm),
            350u64 => parse_o_div_e_u64(asm),
            351u64 => parse_o_mul_i8(asm),
            352u64 => parse_o_mul_i16(asm),
            353u64 => parse_o_mul_i32(asm),
            354u64 => parse_o_mul_i64(asm),
            355u64 => parse_o_mul_u8(asm),
            356u64 => parse_o_mul_u16(asm),
            357u64 => parse_o_mul_u32(asm),
            358u64 => parse_o_mul_u64(asm),
            359u64 => parse_o_neg_i8(asm),
            360u64 => parse_o_neg_i16(asm),
            361u64 => parse_o_neg_i32(asm),
            362u64 => parse_o_neg_i64(asm),
            363u64 => parse_o_pow_i8(asm),
            364u64 => parse_o_pow_i16(asm),
            365u64 => parse_o_pow_i32(asm),
            366u64 => parse_o_pow_i64(asm),
            367u64 => parse_o_pow_u8(asm),
            368u64 => parse_o_pow_u16(asm),
            369u64 => parse_o_pow_u32(asm),
            370u64 => parse_o_pow_u64(asm),
            371u64 => parse_o_rem_i8(asm),
            372u64 => parse_o_rem_i16(asm),
            373u64 => parse_o_rem_i32(asm),
            374u64 => parse_o_rem_i64(asm),
            375u64 => parse_o_rem_u8(asm),
            376u64 => parse_o_rem_u16(asm),
            377u64 => parse_o_rem_u32(asm),
            378u64 => parse_o_rem_u64(asm),
            379u64 => parse_o_rem_e_i8(asm),
            380u64 => parse_o_rem_e_i16(asm),
            381u64 => parse_o_rem_e_i32(asm),
            382u64 => parse_o_rem_e_i64(asm),
            383u64 => parse_o_rem_e_u8(asm),
            384u64 => parse_o_rem_e_u16(asm),
            385u64 => parse_o_rem_e_u32(asm),
            386u64 => parse_o_rem_e_u64(asm),
            387u64 => parse_o_shl_i8(asm),
            388u64 => parse_o_shl_i16(asm),
            389u64 => parse_o_shl_i32(asm),
            390u64 => parse_o_shl_i64(asm),
            391u64 => parse_o_shl_u8(asm),
            392u64 => parse_o_shl_u16(asm),
            393u64 => parse_o_shl_u32(asm),
            394u64 => parse_o_shl_u64(asm),
            395u64 => parse_o_shr_i8(asm),
            396u64 => parse_o_shr_i16(asm),
            397u64 => parse_o_shr_i32(asm),
            398u64 => parse_o_shr_i64(asm),
            399u64 => parse_o_shr_u8(asm),
            400u64 => parse_o_shr_u16(asm),
            401u64 => parse_o_shr_u32(asm),
            402u64 => parse_o_shr_u64(asm),
            403u64 => parse_o_sub_i8(asm),
            404u64 => parse_o_sub_i16(asm),
            405u64 => parse_o_sub_i32(asm),
            406u64 => parse_o_sub_i64(asm),
            407u64 => parse_o_sub_u8(asm),
            408u64 => parse_o_sub_u16(asm),
            409u64 => parse_o_sub_u32(asm),
            410u64 => parse_o_sub_u64(asm),
            411u64 => parse_o_sub_u_i8(asm),
            412u64 => parse_o_sub_u_i16(asm),
            413u64 => parse_o_sub_u_i32(asm),
            414u64 => parse_o_sub_u_i64(asm),
            415u64 => parse_s_abs_i8(asm),
            416u64 => parse_s_abs_i16(asm),
            417u64 => parse_s_abs_i32(asm),
            418u64 => parse_s_abs_i64(asm),
            419u64 => parse_s_add_i8(asm),
            420u64 => parse_s_add_i16(asm),
            421u64 => parse_s_add_i32(asm),
            422u64 => parse_s_add_i64(asm),
            423u64 => parse_s_add_u8(asm),
            424u64 => parse_s_add_u16(asm),
            425u64 => parse_s_add_u32(asm),
            426u64 => parse_s_add_u64(asm),
            427u64 => parse_s_add_u_i8(asm),
            428u64 => parse_s_add_u_i16(asm),
            429u64 => parse_s_add_u_i32(asm),
            430u64 => parse_s_add_u_i64(asm),
            431u64 => parse_s_add_s_u8(asm),
            432u64 => parse_s_add_s_u16(asm),
            433u64 => parse_s_add_s_u32(asm),
            434u64 => parse_s_add_s_u64(asm),
            435u64 => parse_s_div_i8(asm),
            436u64 => parse_s_div_i16(asm),
            437u64 => parse_s_div_i32(asm),
            438u64 => parse_s_div_i64(asm),
            439u64 => parse_s_div_u8(asm),
            440u64 => parse_s_div_u16(asm),
            441u64 => parse_s_div_u32(asm),
            442u64 => parse_s_div_u64(asm),
            443u64 => parse_s_mul_i8(asm),
            444u64 => parse_s_mul_i16(asm),
            445u64 => parse_s_mul_i32(asm),
            446u64 => parse_s_mul_i64(asm),
            447u64 => parse_s_mul_u8(asm),
            448u64 => parse_s_mul_u16(asm),
            449u64 => parse_s_mul_u32(asm),
            450u64 => parse_s_mul_u64(asm),
            451u64 => parse_s_neg_i8(asm),
            452u64 => parse_s_neg_i16(asm),
            453u64 => parse_s_neg_i32(asm),
            454u64 => parse_s_neg_i64(asm),
            455u64 => parse_s_pow_i8(asm),
            456u64 => parse_s_pow_i16(asm),
            457u64 => parse_s_pow_i32(asm),
            458u64 => parse_s_pow_i64(asm),
            459u64 => parse_s_pow_u8(asm),
            460u64 => parse_s_pow_u16(asm),
            461u64 => parse_s_pow_u32(asm),
            462u64 => parse_s_pow_u64(asm),
            463u64 => parse_s_sub_i8(asm),
            464u64 => parse_s_sub_i16(asm),
            465u64 => parse_s_sub_i32(asm),
            466u64 => parse_s_sub_i64(asm),
            467u64 => parse_s_sub_u8(asm),
            468u64 => parse_s_sub_u16(asm),
            469u64 => parse_s_sub_u32(asm),
            470u64 => parse_s_sub_u64(asm),
            471u64 => parse_s_sub_u_i8(asm),
            472u64 => parse_s_sub_u_i16(asm),
            473u64 => parse_s_sub_u_i32(asm),
            474u64 => parse_s_sub_u_i64(asm),
            475u64 => parse_abs_f32(asm),
            476u64 => parse_abs_f64(asm),
            477u64 => parse_add_f32(asm),
            478u64 => parse_add_f64(asm),
            479u64 => parse_div_f32(asm),
            480u64 => parse_div_f64(asm),
            481u64 => parse_div_e_f32(asm),
            482u64 => parse_div_e_f64(asm),
            483u64 => parse_log_f32(asm),
            484u64 => parse_log_f64(asm),
            485u64 => parse_sqrt_f32(asm),
            486u64 => parse_sqrt_f64(asm),
            487u64 => parse_mul_f32(asm),
            488u64 => parse_mul_f64(asm),
            489u64 => parse_neg_f32(asm),
            490u64 => parse_neg_f64(asm),
            491u64 => parse_pow_f32(asm),
            492u64 => parse_pow_f64(asm),
            493u64 => parse_rem_f32(asm),
            494u64 => parse_rem_f64(asm),
            495u64 => parse_rem_e_f32(asm),
            496u64 => parse_rem_e_f64(asm),
            497u64 => parse_cbrt_f32(asm),
            498u64 => parse_cbrt_f64(asm),
            499u64 => parse_sub_f32(asm),
            500u64 => parse_sub_f64(asm),
            501u64 => parse_fetch_add_i8(asm),
            502u64 => parse_fetch_add_i16(asm),
            503u64 => parse_fetch_add_i32(asm),
            504u64 => parse_fetch_add_i64(asm),
            505u64 => parse_fetch_add_u8(asm),
            506u64 => parse_fetch_add_u16(asm),
            507u64 => parse_fetch_add_u32(asm),
            508u64 => parse_fetch_add_u64(asm),
            509u64 => parse_fetch_sub_i8(asm),
            510u64 => parse_fetch_sub_i16(asm),
            511u64 => parse_fetch_sub_i32(asm),
            512u64 => parse_fetch_sub_i64(asm),
            513u64 => parse_fetch_sub_u8(asm),
            514u64 => parse_fetch_sub_u16(asm),
            515u64 => parse_fetch_sub_u32(asm),
            516u64 => parse_fetch_sub_u64(asm),
            517u64 => parse_fetch_min_i8(asm),
            518u64 => parse_fetch_min_i16(asm),
            519u64 => parse_fetch_min_i32(asm),
            520u64 => parse_fetch_min_i64(asm),
            521u64 => parse_fetch_min_u8(asm),
            522u64 => parse_fetch_min_u16(asm),
            523u64 => parse_fetch_min_u32(asm),
            524u64 => parse_fetch_min_u64(asm),
            525u64 => parse_fetch_max_i8(asm),
            526u64 => parse_fetch_max_i16(asm),
            527u64 => parse_fetch_max_i32(asm),
            528u64 => parse_fetch_max_i64(asm),
            529u64 => parse_fetch_max_u8(asm),
            530u64 => parse_fetch_max_u16(asm),
            531u64 => parse_fetch_max_u32(asm),
            532u64 => parse_fetch_max_u64(asm),
            533u64 => parse_fetch_and_i8(asm),
            534u64 => parse_fetch_and_i16(asm),
            535u64 => parse_fetch_and_i32(asm),
            536u64 => parse_fetch_and_i64(asm),
            537u64 => parse_fetch_and_u8(asm),
            538u64 => parse_fetch_and_u16(asm),
            539u64 => parse_fetch_and_u32(asm),
            540u64 => parse_fetch_and_u64(asm),
            541u64 => parse_fetch_nand_i8(asm),
            542u64 => parse_fetch_nand_i16(asm),
            543u64 => parse_fetch_nand_i32(asm),
            544u64 => parse_fetch_nand_i64(asm),
            545u64 => parse_fetch_nand_u8(asm),
            546u64 => parse_fetch_nand_u16(asm),
            547u64 => parse_fetch_nand_u32(asm),
            548u64 => parse_fetch_nand_u64(asm),
            549u64 => parse_fetch_or_i8(asm),
            550u64 => parse_fetch_or_i16(asm),
            551u64 => parse_fetch_or_i32(asm),
            552u64 => parse_fetch_or_i64(asm),
            553u64 => parse_fetch_or_u8(asm),
            554u64 => parse_fetch_or_u16(asm),
            555u64 => parse_fetch_or_u32(asm),
            556u64 => parse_fetch_or_u64(asm),
            557u64 => parse_fetch_xor_i8(asm),
            558u64 => parse_fetch_xor_i16(asm),
            559u64 => parse_fetch_xor_i32(asm),
            560u64 => parse_fetch_xor_i64(asm),
            561u64 => parse_fetch_xor_u8(asm),
            562u64 => parse_fetch_xor_u16(asm),
            563u64 => parse_fetch_xor_u32(asm),
            564u64 => parse_fetch_xor_u64(asm),
            565u64 => parse_cmp_exchg_i8(asm),
            566u64 => parse_cmp_exchg_i16(asm),
            567u64 => parse_cmp_exchg_i32(asm),
            568u64 => parse_cmp_exchg_i64(asm),
            569u64 => parse_cmp_exchg_u8(asm),
            570u64 => parse_cmp_exchg_u16(asm),
            571u64 => parse_cmp_exchg_u32(asm),
            572u64 => parse_cmp_exchg_u64(asm),
            573u64 => parse_atomic_ld_8(asm),
            574u64 => parse_atomic_ld_16(asm),
            575u64 => parse_atomic_ld_32(asm),
            576u64 => parse_atomic_ld_64(asm),
            577u64 => parse_atomic_st_8(asm),
            578u64 => parse_atomic_st_16(asm),
            579u64 => parse_atomic_st_32(asm),
            580u64 => parse_atomic_st_64(asm),
            581u64 => parse_spawn(asm),
            582u64 => parse_wait(asm),
            583u64 => parse_notify(asm),
            584u64 => parse_swap_8(asm),
            585u64 => parse_swap_16(asm),
            586u64 => parse_swap_32(asm),
            587u64 => parse_swap_64(asm),
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
fn parse_imm<'source>(asm: &mut Assembler<'source>) -> u64 {
    if asm.matches(Token_Kind::Colon) {
        asm.expects(Token_Kind::Identifier);
        asm.patch_label() as u64
    } else if asm.matches(Token_Kind::Ampersand) {
        asm.expects(Token_Kind::Identifier);
        asm.patch_address() as u64
    } else {
        asm.expects(Token_Kind::Number);
        unsafe { asm.entry().value.integer }
    }
}
#[inline(always)]
fn parse_u16<'source>(asm: &mut Assembler<'source>) -> u16 {
    parse_imm(asm) as u16
}
#[inline(always)]
fn parse_i16<'source>(asm: &mut Assembler<'source>) -> i16 {
    parse_imm(asm) as i16
}
#[inline(always)]
fn parse_i32<'source>(asm: &mut Assembler<'source>) -> i32 {
    parse_imm(asm) as i32
}
fn parse_halt<'source>(asm: &mut Assembler<'source>) {
    let instr = { Instruction::Halt };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_trap<'source>(asm: &mut Assembler<'source>) {
    let instr = { Instruction::Trap };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_call<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i32(asm);
        Instruction::Call { imm }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_ret<'source>(asm: &mut Assembler<'source>) {
    let instr = { Instruction::Ret };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_ecall<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i32(asm);
        Instruction::Ecall { imm }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_jal<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i32(asm);
        Instruction::Jal { imm }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bie<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bie { imm, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bne<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bne { imm, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_blts<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Blts { imm, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bltu<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bltu { imm, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bles<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bles { imm, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bleu<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bleu { imm, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bgts<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bgts { imm, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bgtu<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bgtu { imm, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bges<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bges { imm, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bgeu<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bgeu { imm, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bie_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bie_f32 {
            imm,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bie_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bie_f64 {
            imm,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bne_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bne_f32 {
            imm,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bne_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bne_f64 {
            imm,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_blt_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Blt_f32 {
            imm,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_blt_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Blt_f64 {
            imm,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_ble_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Ble_f32 {
            imm,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_ble_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Ble_f64 {
            imm,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bgt_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bgt_f32 {
            imm,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bgt_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bgt_f64 {
            imm,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bge_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bge_f32 {
            imm,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_bge_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i16(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Bge_f64 {
            imm,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cie<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cie { rd, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cie_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cie_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cie_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cie_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cne<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cne { rd, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cne_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cne_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cne_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cne_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_clts<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Clts { rd, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cltu<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cltu { rd, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_clt_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Clt_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_clt_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Clt_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cles<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cles { rd, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cleu<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cleu { rd, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cle_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cle_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cle_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cle_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cgts<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cgts { rd, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cgtu<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cgtu { rd, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cgt_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cgt_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cgt_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cgt_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cges<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cges { rd, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cgeu<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cgeu { rd, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cge_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cge_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cge_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cge_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_lra_8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Lra_8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_lra_16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Lra_16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_lra_32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Lra_32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_lra_64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Lra_64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_lsi<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let imm = parse_i16(asm);
        Instruction::Lsi { rd, imm }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_lui<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let imm = parse_u16(asm);
        Instruction::Lui { rd, imm }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_lia_8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let imm = parse_u16(asm);
        Instruction::Lia_8 { rd, imm }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_lia_16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let imm = parse_u16(asm);
        Instruction::Lia_16 { rd, imm }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_lia_32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let imm = parse_u16(asm);
        Instruction::Lia_32 { rd, imm }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_lia_64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let imm = parse_u16(asm);
        Instruction::Lia_64 { rd, imm }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_sra_8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Sra_8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_sra_16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Sra_16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_sra_32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Sra_32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_sra_64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Sra_64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_ssi<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let imm = parse_i16(asm);
        Instruction::Ssi { rd, imm }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_sui<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let imm = parse_u16(asm);
        Instruction::Sui { rd, imm }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_mov<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Mov { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_pop<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        Instruction::Pop { rd }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_psh<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        Instruction::Psh { rd }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_psi<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i32(asm);
        Instruction::Psi { imm }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_pui<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let imm = parse_i32(asm);
        Instruction::Pui { imm }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_and_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::And_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_and_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::And_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_and_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::And_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_and_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::And_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_and_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::And_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_and_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::And_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_and_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::And_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_and_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::And_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_or_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Or_i8 { rd, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_or_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Or_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_or_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Or_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_or_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Or_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_or_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Or_u8 { rd, rs1, rs2 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_or_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Or_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_or_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Or_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_or_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Or_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_xor_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Xor_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_xor_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Xor_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_xor_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Xor_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_xor_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Xor_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_xor_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Xor_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_xor_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Xor_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_xor_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Xor_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_xor_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Xor_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_not_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Not_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_not_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Not_i16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_not_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Not_i32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_not_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Not_i64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_not_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Not_u8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_not_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Not_u16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_not_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Not_u32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_not_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Not_u64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shl_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shl_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shl_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shl_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shl_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shl_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shl_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shl_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shl_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shl_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shl_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shl_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shl_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shl_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shl_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shl_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shr_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shr_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shr_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shr_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shr_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shr_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shr_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shr_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shr_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shr_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shr_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shr_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shr_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shr_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_shr_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Shr_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_l_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_L_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_l_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_L_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_l_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_L_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_l_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_L_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_l_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_L_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_l_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_L_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_l_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_L_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_l_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_L_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_r_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_R_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_r_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_R_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_r_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_R_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_r_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_R_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_r_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_R_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_r_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_R_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_r_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_R_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rot_r_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rot_R_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_ones_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Ones_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_ones_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Ones_i16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_ones_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Ones_i32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_ones_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Ones_i64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_ones_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Ones_u8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_ones_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Ones_u16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_ones_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Ones_u32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_ones_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Ones_u64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_ones_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Ones_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_ones_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Ones_i16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_ones_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Ones_i32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_ones_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Ones_i64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_ones_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Ones_u8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_ones_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Ones_u16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_ones_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Ones_u32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_ones_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Ones_u64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_ones_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Ones_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_ones_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Ones_i16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_ones_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Ones_i32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_ones_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Ones_i64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_ones_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Ones_u8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_ones_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Ones_u16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_ones_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Ones_u32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_ones_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Ones_u64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_zeros_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Zeros_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_zeros_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Zeros_i16 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_zeros_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Zeros_i32 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_zeros_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Zeros_i64 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_zeros_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Zeros_u8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_zeros_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Zeros_u16 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_zeros_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Zeros_u32 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_zeros_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Zeros_u64 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_zeros_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Zeros_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_zeros_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Zeros_i16 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_zeros_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Zeros_i32 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_zeros_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Zeros_i64 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_zeros_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Zeros_u8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_zeros_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Zeros_u16 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_zeros_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Zeros_u32 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_l_zeros_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::L_Zeros_u64 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_zeros_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Zeros_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_zeros_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Zeros_i16 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_zeros_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Zeros_i32 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_zeros_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Zeros_i64 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_zeros_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Zeros_u8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_zeros_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Zeros_u16 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_zeros_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Zeros_u32 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_t_zeros_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::T_Zeros_u64 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bytes_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bytes_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bytes_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bytes_i16 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bytes_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bytes_i32 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bytes_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bytes_i64 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bytes_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bytes_u8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bytes_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bytes_u16 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bytes_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bytes_u32 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bytes_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bytes_u64 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bits_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bits_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bits_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bits_i16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bits_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bits_i32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bits_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bits_i64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bits_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bits_u8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bits_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bits_u16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bits_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bits_u32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_r_bits_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::R_Bits_u64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_abs_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Abs_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_abs_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Abs_i16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_abs_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Abs_i32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_abs_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Abs_i64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_u_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_U_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_u_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_U_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_u_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_U_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_u_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_U_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_s_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_S_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_s_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_S_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_s_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_S_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_add_s_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Add_S_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_e_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_E_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_e_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_E_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_e_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_E_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_e_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_E_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_e_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_E_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_e_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_E_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_e_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_E_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_div_e_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Div_E_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_log_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Log_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_log_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Log_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_log_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Log_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_log_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Log_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_log_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Log_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_log_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Log_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_log_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Log_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_log_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Log_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sqrt_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Sqrt_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sqrt_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Sqrt_i16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sqrt_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Sqrt_i32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sqrt_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Sqrt_i64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sqrt_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Sqrt_u8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sqrt_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Sqrt_u16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sqrt_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Sqrt_u32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sqrt_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Sqrt_u64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_mul_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Mul_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_mul_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Mul_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_mul_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Mul_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_mul_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Mul_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_mul_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Mul_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_mul_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Mul_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_mul_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Mul_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_mul_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Mul_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_neg_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Neg_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_neg_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Neg_i16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_neg_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Neg_i32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_neg_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::C_Neg_i64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_pow_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Pow_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_pow_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Pow_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_pow_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Pow_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_pow_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Pow_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_pow_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Pow_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_pow_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Pow_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_pow_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Pow_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_pow_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Pow_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_e_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_E_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_e_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_E_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_e_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_E_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_e_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_E_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_e_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_E_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_e_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_E_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_e_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_E_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_rem_e_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Rem_E_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shl_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shl_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shl_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shl_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shl_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shl_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shl_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shl_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shl_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shl_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shl_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shl_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shl_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shl_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shl_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shl_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shr_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shr_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shr_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shr_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shr_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shr_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shr_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shr_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shr_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shr_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shr_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shr_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shr_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shr_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_shr_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Shr_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sub_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Sub_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sub_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Sub_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sub_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Sub_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sub_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Sub_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sub_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Sub_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sub_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Sub_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sub_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Sub_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sub_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Sub_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sub_u_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Sub_U_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sub_u_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Sub_U_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sub_u_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Sub_U_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_c_sub_u_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::C_Sub_U_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_abs_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::O_Abs_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_abs_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::O_Abs_i16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_abs_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::O_Abs_i32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_abs_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::O_Abs_i64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_u_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_U_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_u_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_U_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_u_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_U_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_u_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_U_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_s_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_S_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_s_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_S_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_s_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_S_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_add_s_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Add_S_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_e_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_E_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_e_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_E_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_e_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_E_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_e_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_E_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_e_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_E_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_e_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_E_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_e_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_E_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_div_e_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Div_E_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_mul_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Mul_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_mul_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Mul_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_mul_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Mul_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_mul_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Mul_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_mul_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Mul_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_mul_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Mul_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_mul_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Mul_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_mul_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Mul_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_neg_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::O_Neg_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_neg_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::O_Neg_i16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_neg_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::O_Neg_i32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_neg_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::O_Neg_i64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_pow_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Pow_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_pow_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Pow_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_pow_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Pow_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_pow_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Pow_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_pow_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Pow_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_pow_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Pow_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_pow_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Pow_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_pow_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Pow_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_e_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_E_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_e_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_E_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_e_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_E_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_e_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_E_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_e_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_E_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_e_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_E_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_e_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_E_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_rem_e_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Rem_E_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shl_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shl_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shl_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shl_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shl_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shl_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shl_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shl_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shl_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shl_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shl_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shl_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shl_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shl_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shl_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shl_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shr_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shr_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shr_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shr_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shr_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shr_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shr_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shr_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shr_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shr_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shr_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shr_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shr_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shr_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_shr_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Shr_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_sub_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Sub_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_sub_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Sub_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_sub_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Sub_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_sub_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Sub_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_sub_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Sub_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_sub_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Sub_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_sub_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Sub_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_sub_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Sub_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_sub_u_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Sub_U_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_sub_u_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Sub_U_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_sub_u_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Sub_U_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_o_sub_u_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::O_Sub_U_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_abs_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::S_Abs_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_abs_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::S_Abs_i16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_abs_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::S_Abs_i32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_abs_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::S_Abs_i64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_u_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_U_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_u_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_U_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_u_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_U_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_u_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_U_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_s_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_S_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_s_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_S_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_s_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_S_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_add_s_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Add_S_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_div_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Div_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_div_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Div_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_div_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Div_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_div_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Div_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_div_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Div_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_div_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Div_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_div_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Div_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_div_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Div_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_mul_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Mul_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_mul_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Mul_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_mul_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Mul_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_mul_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Mul_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_mul_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Mul_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_mul_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Mul_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_mul_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Mul_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_mul_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Mul_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_neg_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::S_Neg_i8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_neg_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::S_Neg_i16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_neg_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::S_Neg_i32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_neg_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::S_Neg_i64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_pow_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Pow_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_pow_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Pow_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_pow_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Pow_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_pow_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Pow_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_pow_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Pow_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_pow_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Pow_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_pow_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Pow_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_pow_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Pow_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_sub_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Sub_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_sub_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Sub_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_sub_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Sub_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_sub_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Sub_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_sub_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Sub_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_sub_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Sub_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_sub_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Sub_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_sub_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Sub_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_sub_u_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Sub_U_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_sub_u_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Sub_U_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_sub_u_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Sub_U_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_s_sub_u_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::S_Sub_U_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_abs_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Abs_f32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_abs_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Abs_f64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_add_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Add_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_add_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Add_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_div_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Div_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_div_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Div_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_div_e_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Div_E_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_div_e_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Div_E_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_log_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Log_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_log_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Log_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_sqrt_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Sqrt_f32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_sqrt_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Sqrt_f64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_mul_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Mul_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_mul_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Mul_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_neg_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Neg_f32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_neg_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Neg_f64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_pow_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Pow_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_pow_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Pow_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rem_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rem_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rem_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rem_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rem_e_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rem_E_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_rem_e_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Rem_E_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cbrt_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Cbrt_f32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cbrt_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Cbrt_f64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_sub_f32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Sub_f32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_sub_f64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Sub_f64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_add_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Add_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_add_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Add_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_add_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Add_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_add_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Add_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_add_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Add_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_add_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Add_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_add_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Add_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_add_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Add_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_sub_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Sub_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_sub_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Sub_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_sub_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Sub_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_sub_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Sub_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_sub_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Sub_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_sub_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Sub_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_sub_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Sub_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_sub_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Sub_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_min_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Min_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_min_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Min_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_min_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Min_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_min_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Min_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_min_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Min_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_min_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Min_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_min_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Min_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_min_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Min_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_max_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Max_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_max_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Max_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_max_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Max_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_max_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Max_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_max_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Max_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_max_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Max_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_max_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Max_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_max_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Max_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_and_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_And_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_and_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_And_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_and_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_And_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_and_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_And_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_and_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_And_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_and_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_And_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_and_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_And_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_and_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_And_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_nand_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Nand_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_nand_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Nand_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_nand_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Nand_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_nand_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Nand_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_nand_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Nand_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_nand_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Nand_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_nand_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Nand_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_nand_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Nand_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_or_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Or_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_or_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Or_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_or_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Or_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_or_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Or_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_or_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Or_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_or_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Or_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_or_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Or_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_or_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Or_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_xor_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Xor_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_xor_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Xor_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_xor_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Xor_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_xor_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Xor_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_xor_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Xor_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_xor_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Xor_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_xor_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Xor_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_fetch_xor_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Fetch_Xor_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cmp_exchg_i8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cmp_Exchg_i8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cmp_exchg_i16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cmp_Exchg_i16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cmp_exchg_i32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cmp_Exchg_i32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cmp_exchg_i64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cmp_Exchg_i64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cmp_exchg_u8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cmp_Exchg_u8 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cmp_exchg_u16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cmp_Exchg_u16 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cmp_exchg_u32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cmp_Exchg_u32 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_cmp_exchg_u64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs2 = parse_register(asm);
        Instruction::Cmp_Exchg_u64 {
            rd,
            rs1,
            rs2,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_atomic_ld_8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Atomic_Ld_8 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_atomic_ld_16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Atomic_Ld_16 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_atomic_ld_32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Atomic_Ld_32 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_atomic_ld_64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Atomic_Ld_64 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_atomic_st_8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Atomic_St_8 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_atomic_st_16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Atomic_St_16 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_atomic_st_32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Atomic_St_32 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_atomic_st_64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Atomic_St_64 {
            rd,
            rs1,
        }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_spawn<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let imm = parse_u16(asm);
        Instruction::Spawn { rd, imm }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_wait<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Wait { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_notify<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Notify { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_swap_8<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Swap_8 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_swap_16<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Swap_16 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_swap_32<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Swap_32 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
fn parse_swap_64<'source>(asm: &mut Assembler<'source>) {
    let instr = {
        let rd = parse_register(asm);
        asm.expects(Token_Kind::Comma);
        let rs1 = parse_register(asm);
        Instruction::Swap_64 { rd, rs1 }
    };
    asm.object.code_instrs.push(instr.encode());
    asm.expects(Token_Kind::Newline);
}
