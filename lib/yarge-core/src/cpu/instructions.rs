use crate::{ Res, YargeError };
use super::{ Cpu, CpuState };

mod macros;
mod macros_cb;
use macros::*;
use macros_cb::*;

pub fn cpu_instructions(cpu: &mut Cpu, instr: u8) -> Res<()> {
  match instr {
    0x00 => { /*IS A NO-OP*/ },             //NOP
    0x01 => { ld_rr_u16!(cpu, BC); },       //LD BC,u16
    0x02 => { ld_mrr_a!(cpu, BC); },        //LD (BC),A
    0x03 => { incdec_rr!(cpu, BC, add); }   //INC BC
    0x04 => { inc_r!(cpu, B); }             //INC B
    0x05 => { dec_r!(cpu, B); }             //DEC B
    0x06 => { ld_r_u8!(cpu, B); }           //LD B,u8
    0x07 => { rlca!(cpu); }                 //RLCA
    0x08 => { ld_mu16_sp!(cpu); }           //LD (u16),SP
    0x09 => { add_hl_rr!(cpu, BC); }        //ADD HL,BC
    0x0A => { ld_a_mrr!(cpu, BC); }         //LD A,(BC)
    0x0B => { incdec_rr!(cpu, BC, sub); }   //DEC BC
    0x0C => { inc_r!(cpu, C); }             //INC C
    0x0D => { dec_r!(cpu, C); }             //DEC C
    0x0E => { ld_r_u8!(cpu, C); }           //LD C,u8 
    0x0F => { rrca!(cpu); }                 //RRCA

    0x10 => { cpu_stop!(cpu); }             //STOP
    0x11 => { ld_rr_u16!(cpu, DE); },       //LD DE,u16
    0x12 => { ld_mrr_a!(cpu, DE); },        //LD (DE),A
    0x13 => { incdec_rr!(cpu, DE, add); },  //INC DE
    0x14 => { inc_r!(cpu, D); }             //INC D
    0x15 => { dec_r!(cpu, D); }             //DEC D
    0x16 => { ld_r_u8!(cpu, D); }           //LD D,u8
    0x17 => { rla!(cpu); }                  //RLA
    0x18 => { jr_i8!(cpu); }                //JR i8
    0x19 => { add_hl_rr!(cpu, DE); }        //ADD HL,DE
    0x1A => { ld_a_mrr!(cpu, DE); }         //LD A,(DE)
    0x1B => { incdec_rr!(cpu, DE, sub); }   //DEC DE
    0x1C => { inc_r!(cpu, E); }             //INC E
    0x1D => { dec_r!(cpu, E); }             //DEC E
    0x1E => { ld_r_u8!(cpu, E); }           //LD E,u8 
    0x1F => { rra!(cpu); }                  //RRA

    0x20 => { jr_i8_cond!(cpu, NZ); }       //JR NZ, i8
    0x21 => { ld_rr_u16!(cpu, HL); },       //LD HL,u16
    0x22 => { ld_mhli_a!(cpu, add); },      //LD (HL+),A
    0x23 => { incdec_rr!(cpu, HL, add); },  //INC HL
    0x24 => { inc_r!(cpu, H); }             //INC H
    0x25 => { dec_r!(cpu, H); }             //DEC H
    0x26 => { ld_r_u8!(cpu, H); }           //LD H,u8
    0x27 => { daa!(cpu); }                  //DAA
    0x28 => { jr_i8_cond!(cpu, Z); }        //JR Z, i8 
    0x29 => { add_hl_rr!(cpu, HL); }        //ADD HL,HL
    0x2A => { ld_a_mhli!(cpu, add); }       //LD A,(HL+)
    0x2B => { incdec_rr!(cpu, HL, sub); }   //DEC HL
    0x2C => { inc_r!(cpu, L); }             //INC L
    0x2D => { dec_r!(cpu, L); }             //DEC L
    0x2E => { ld_r_u8!(cpu, L); }           //LD L,u8 
    0x2F => { cpl!(cpu); }                  //CPL

    0x30 => { jr_i8_cond!(cpu, NC); }       //JR NZ, i8
    0x31 => { ld_rr_u16!(cpu, SP); },       //LD SP,u16
    0x32 => { ld_mhli_a!(cpu, sub); },      //LD (HL-),A
    0x33 => { incdec_rr!(cpu, SP, add); },  //INC SP
    0x34 => { inc_mhl!(cpu); }              //INC (HL)
    0x35 => { dec_mhl!(cpu); }              //DEC (HL)
    0x36 => { ld_mhl_u8!(cpu); }            //LD (HL), u8
    0x37 => { scf!(cpu); }                  //SCF
    0x38 => { jr_i8_cond!(cpu, C); }        //JR C, i8 
    0x39 => { add_hl_rr!(cpu, SP); }        //ADD HL,SP
    0x3A => { ld_a_mhli!(cpu, sub); }       //LD A,(HL-)
    0x3B => { incdec_rr!(cpu, SP, sub); },  //DEC SP
    0x3C => { inc_r!(cpu, A); }             //INC A
    0x3D => { dec_r!(cpu, A); }             //DEC A
    0x3E => { ld_r_u8!(cpu, A); }           //LD A,u8 
    0x3F => { ccf!(cpu); }                  //CCF

    0x40 => { /*TODO Breakpoint */ }          //LD B,B
    0x41 => { ld_r_r!(cpu, B, C); }         //LD B,C
    0x42 => { ld_r_r!(cpu, B, D); }         //LD B,D
    0x43 => { ld_r_r!(cpu, B, E); }         //LD B,E
    0x44 => { ld_r_r!(cpu, B, H); }         //LD B,H
    0x45 => { ld_r_r!(cpu, B, L); }         //LD B,L
    0x46 => { ld_r_mhl!(cpu, B); }          //LD B,(HL)
    0x47 => { ld_r_r!(cpu, B, A); }         //LD B,A
    0x48 => { ld_r_r!(cpu, C, B); }         //LD C,B
    0x49 => { /*IS A NO-OP*/ }                //LD C,C
    0x4A => { ld_r_r!(cpu, C, D); }         //LD C,D
    0x4B => { ld_r_r!(cpu, C, E); }         //LD C,E
    0x4C => { ld_r_r!(cpu, C, H); }         //LD C,H
    0x4D => { ld_r_r!(cpu, C, L); }         //LD C,L
    0x4E => { ld_r_mhl!(cpu, C); }          //LD C,(HL)
    0x4F => { ld_r_r!(cpu, C, A); }         //LD C,A

    0x50 => { ld_r_r!(cpu, D, B); }         //LD D,B
    0x51 => { ld_r_r!(cpu, D, C); }         //LD D,C
    0x52 => { /*IS A NO-OP*/ }                //LD D,D
    0x53 => { ld_r_r!(cpu, D, E); }         //LD D,E
    0x54 => { ld_r_r!(cpu, D, H); }         //LD D,H
    0x55 => { ld_r_r!(cpu, D, L); }         //LD D,L
    0x56 => { ld_r_mhl!(cpu, D); }          //LD D,(HL)
    0x57 => { ld_r_r!(cpu, D, A); }         //LD D,A
    0x58 => { ld_r_r!(cpu, E, B); }         //LD E,B
    0x59 => { ld_r_r!(cpu, E, C); }         //LD E,C
    0x5A => { ld_r_r!(cpu, E, D); }         //LD E,D
    0x5B => { /*IS A NO-OP*/ }                //LD E,E
    0x5C => { ld_r_r!(cpu, E, H); }         //LD E,H
    0x5D => { ld_r_r!(cpu, E, L); }         //LD E,L
    0x5E => { ld_r_mhl!(cpu, E); }          //LD E,(HL)
    0x5F => { ld_r_r!(cpu, E, A); }         //LD E,A

    0x60 => { ld_r_r!(cpu, H, B); }         //LD H,B
    0x61 => { ld_r_r!(cpu, H, C); }         //LD H,C
    0x62 => { ld_r_r!(cpu, H, D); }         //LD H,D
    0x63 => { ld_r_r!(cpu, H, E); }         //LD H,E
    0x64 => { /*IS A NO-OP*/ }                //LD H,H
    0x65 => { ld_r_r!(cpu, H, L); }         //LD H,L
    0x66 => { ld_r_mhl!(cpu, H); }          //LD H,(HL)
    0x67 => { ld_r_r!(cpu, H, A); }         //LD H,A
    0x68 => { ld_r_r!(cpu, L, B); }         //LD L,B
    0x69 => { ld_r_r!(cpu, L, C); }         //LD L,C
    0x6A => { ld_r_r!(cpu, L, D); }         //LD L,D
    0x6B => { ld_r_r!(cpu, L, E); }         //LD L,E
    0x6C => { ld_r_r!(cpu, L, H); }         //LD L,H
    0x6D => { /*IS A NO-OP*/ }                //LD L,L
    0x6E => { ld_r_mhl!(cpu, L); }          //LD L,(HL)
    0x6F => { ld_r_r!(cpu, L, A); }         //LD L,A
    
    0x70 => { ld_mhl_r!(cpu, B); }          //LD (HL),B
    0x71 => { ld_mhl_r!(cpu, C); }          //LD (HL),C
    0x72 => { ld_mhl_r!(cpu, D); }          //LD (HL),D
    0x73 => { ld_mhl_r!(cpu, E); }          //LD (HL),E
    0x74 => { ld_mhl_r!(cpu, H); }          //LD (HL),H
    0x75 => { ld_mhl_r!(cpu, L); }          //LD (HL),L
    0x76 => { cpu_halt!(cpu); }             //HALT
    0x77 => { ld_mhl_r!(cpu, A); }          //LD (HL),A
    0x78 => { ld_r_r!(cpu, A, B); }         //LD A,B
    0x79 => { ld_r_r!(cpu, A, C); }         //LD A,C
    0x7A => { ld_r_r!(cpu, A, D); }         //LD A,D
    0x7B => { ld_r_r!(cpu, A, E); }         //LD A,E
    0x7C => { ld_r_r!(cpu, A, H); }         //LD A,H
    0x7D => { ld_r_r!(cpu, A, L); }         //LD A,L
    0x7E => { ld_r_mhl!(cpu, A); }          //LD A,(HL)
    0x7F => { /*IS A NO-OP*/ }                //LD A,A

    0x80 => { add_a_r!(cpu, B); }           //ADD A,B
    0x81 => { add_a_r!(cpu, C); }           //ADD A,C
    0x82 => { add_a_r!(cpu, D); }           //ADD A,D
    0x83 => { add_a_r!(cpu, E); }           //ADD A,E
    0x84 => { add_a_r!(cpu, H); }           //ADD A,H
    0x85 => { add_a_r!(cpu, L); }           //ADD A,L
    0x86 => { add_a_mhl!(cpu); }            //ADD A,(HL)
    0x87 => { add_a_r!(cpu, A); }           //ADD A,A
    0x88 => { adc_a_r!(cpu, B); }           //ADC A,B
    0x89 => { adc_a_r!(cpu, C); }           //ADC A,C
    0x8A => { adc_a_r!(cpu, D); }           //ADC A,D
    0x8B => { adc_a_r!(cpu, E); }           //ADC A,E
    0x8C => { adc_a_r!(cpu, H); }           //ADC A,H
    0x8D => { adc_a_r!(cpu, L); }           //ADC A,L
    0x8E => { adc_a_mhl!(cpu); }            //ADC A,(HL)
    0x8F => { adc_a_r!(cpu, A); }           //ADC A,A

    0x90 => { sub_a_r!(cpu, B); }           //SUB A,B
    0x91 => { sub_a_r!(cpu, C); }           //SUB A,C
    0x92 => { sub_a_r!(cpu, D); }           //SUB A,D
    0x93 => { sub_a_r!(cpu, E); }           //SUB A,E
    0x94 => { sub_a_r!(cpu, H); }           //SUB A,H
    0x95 => { sub_a_r!(cpu, L); }           //SUB A,L
    0x96 => { sub_a_mhl!(cpu); }            //SUB A,(HL)
    0x97 => { sub_a_r!(cpu, A); }           //SUB A,A
    0x98 => { sbc_a_r!(cpu, B); }           //SBC A,B
    0x99 => { sbc_a_r!(cpu, C); }           //SBC A,C
    0x9A => { sbc_a_r!(cpu, D); }           //SBC A,D
    0x9B => { sbc_a_r!(cpu, E); }           //SBC A,E
    0x9C => { sbc_a_r!(cpu, H); }           //SBC A,H
    0x9D => { sbc_a_r!(cpu, L); }           //SBC A,L
    0x9E => { sbc_a_mhl!(cpu); }            //SBC A,(HL)
    0x9F => { sbc_a_r!(cpu, A); }           //SBC A,A
    
    0xA0 => { and_a_r!(cpu, B); }           //AND A,B
    0xA1 => { and_a_r!(cpu, C); }           //AND A,C
    0xA2 => { and_a_r!(cpu, D); }           //AND A,D
    0xA3 => { and_a_r!(cpu, E); }           //AND A,E
    0xA4 => { and_a_r!(cpu, H); }           //AND A,H
    0xA5 => { and_a_r!(cpu, L); }           //AND A,L
    0xA6 => { and_a_mhl!(cpu); }            //AND A,(HL)
    0xA7 => { and_a_r!(cpu, A); }           //AND A,A
    0xA8 => { xor_a_r!(cpu, B); }           //XOR A,B
    0xA9 => { xor_a_r!(cpu, C); }           //XOR A,C
    0xAA => { xor_a_r!(cpu, D); }           //XOR A,D
    0xAB => { xor_a_r!(cpu, E); }           //XOR A,E
    0xAC => { xor_a_r!(cpu, H); }           //XOR A,H
    0xAD => { xor_a_r!(cpu, L); }           //XOR A,L
    0xAE => { xor_a_mhl!(cpu); }            //XOR A,(HL)
    0xAF => { xor_a_r!(cpu, A); }           //XOR A,A

    0xB0 => { or_a_r!(cpu, B); }            //OR A,B
    0xB1 => { or_a_r!(cpu, C); }            //OR A,C
    0xB2 => { or_a_r!(cpu, D); }            //OR A,D
    0xB3 => { or_a_r!(cpu, E); }            //OR A,E
    0xB4 => { or_a_r!(cpu, H); }            //OR A,H
    0xB5 => { or_a_r!(cpu, L); }            //OR A,L
    0xB6 => { or_a_mhl!(cpu); }             //OR A,(HL)
    0xB7 => { or_a_r!(cpu, A); }            //OR A,A
    0xB8 => { cp_a_r!(cpu, B); }            //CP A,B
    0xB9 => { cp_a_r!(cpu, C); }            //CP A,C
    0xBA => { cp_a_r!(cpu, D); }            //CP A,D
    0xBB => { cp_a_r!(cpu, E); }            //CP A,E
    0xBC => { cp_a_r!(cpu, H); }            //CP A,H
    0xBD => { cp_a_r!(cpu, L); }            //CP A,L
    0xBE => { cp_a_mhl!(cpu); }             //CP A,(HL)
    0xBF => { cp_a_r!(cpu, A); }            //CP A,A

    0xC0 => { ret_cond!(cpu, NZ); }         //RET NZ
    0xC1 => { pop_rr!(cpu, BC); }           //POP BC
    0xC2 => { cond_jp_u16!(cpu, NZ); }      //JP NZ,u16
    0xC3 => { jp_u16!(cpu); }               //JP u16
    0xC4 => { call_u16_cond!(cpu, NZ); }    //CALL NZ,u16
    0xC5 => { push_rr!(cpu, BC); }          //PUSH BC
    0xC6 => { add_a_u8!(cpu); }             //ADD A,u8
    0xC7 => { rst!(cpu, 0x00); }            //RST 00h
    0xC8 => { ret_cond!(cpu, Z); }          //RET Z
    0xC9 => { ret!(cpu); }                  //RET
    0xCA => { cond_jp_u16!(cpu, Z); }       //JP Z,u16
    0xCC => { call_u16_cond!(cpu, Z); }     //CALL Z,u16
    0xCD => { call_u16!(cpu); }             //CALL u16
    0xCE => { adc_a_u8!(cpu); }             //ADC A,u8
    0xCF => { rst!(cpu, 0x08); }            //RST 08h

    0xD0 => { ret_cond!(cpu, NC); }         //RET NC
    0xD1 => { pop_rr!(cpu, DE); }           //POP DE
    0xD2 => { cond_jp_u16!(cpu, NC); }      //JP NC,u16
    0xD4 => { call_u16_cond!(cpu, NC); }    //CALL NZ,u16
    0xD5 => { push_rr!(cpu, DE); }          //PUSH DE
    0xD6 => { sub_a_u8!(cpu); }             //SUB A,u8
    0xD7 => { rst!(cpu, 0x10); }            //RST 10h
    0xD8 => { ret_cond!(cpu, C); }          //RET C
    0xD9 => { reti!(cpu); }                 //RETI
    0xDA => { cond_jp_u16!(cpu, C); }       //JP C,u16
    0xDC => { call_u16_cond!(cpu, C); }     //CALL C,u16
    0xDE => { sbc_a_u8!(cpu); }             //SBC A,u8
    0xDF => { rst!(cpu, 0x18); }            //RST 18h

    0xE0 => { ld_m_ff00_add_u8_a!(cpu); }   //LD (FFOO+u8),A
    0xE1 => { pop_rr!(cpu, HL); }           //POP HL
    0xE2 => { ld_m_ff00_add_c_a!(cpu); }    //LD (FF00+C),A
    0xE5 => { push_rr!(cpu, HL); }          //PUSH HL
    0xE6 => { and_a_u8!(cpu); }             //AND A,u8
    0xE7 => { rst!(cpu, 0x20); }            //RST 20h
    0xE8 => { add_sp_i8!(cpu); }            //ADD SP,i8
    0xE9 => { jp_hl!(cpu); }                //JP HL
    0xEA => { ld_mu16_a!(cpu); }            //LD (u16),A
    0xEE => { xor_a_u8!(cpu); }             //XOR A,u8
    0xEF => { rst!(cpu, 0x28); }            //RST 28h

    0xF0 => { ld_a_m_ff00_add_u8!(cpu); }   //LD A,(FF00+u8)
    0xF1 => { pop_rr!(cpu, AF); }           //POP AF
    0xF2 => { ld_a_m_ff00_add_c!(cpu); }    //LD A,(FF00+C)
    0xF3 => { di!(cpu); }                   //DI
    0xF5 => { push_rr!(cpu, AF); }          //PUSH AF
    0xF6 => { or_a_u8!(cpu); }              //OR A,u8
    0xF7 => { rst!(cpu, 0x30); }            //RST 30h
    0xF8 => { ld_hl_sp_i8!(cpu); }          //LD HL,SP+i8
    0xF9 => { ld_sp_hl!(cpu); }             //LD SP,HL
    0xFA => { ld_a_mu16!(cpu); }            //LD A,(u16)
    0xFB => { ei!(cpu); }                   //EI
    0xFE => { cp_a_u8!(cpu); }              //CP A,u8
    0xFF => { rst!(cpu, 0x38); }            //RST 38h

    _ => { 
      Err(YargeError::InvalidInstruction{
        addr: cpu.reg.pc.wrapping_sub(1),
        instr: instr
      })?;
    }
  }
  Ok(())
}

pub fn cpu_instructions_cb(cpu: &mut Cpu, instr: u8) -> Res<()> {
  match instr {
    0x00 => { rlc_r!(cpu, B); }             // RLC B
    0x01 => { rlc_r!(cpu, C); }             // RLC C
    0x02 => { rlc_r!(cpu, D); }             // RLC D
    0x03 => { rlc_r!(cpu, E); }             // RLC E
    0x04 => { rlc_r!(cpu, H); }             // RLC H
    0x05 => { rlc_r!(cpu, L); }             // RLC L
    0x06 => { rlc_mhl!(cpu); }              // RLC (HL)
    0x07 => { rlc_r!(cpu, A); }             // RLC A
    0x08 => { rrc_r!(cpu, B); }             // RRC B
    0x09 => { rrc_r!(cpu, C); }             // RRC C
    0x0A => { rrc_r!(cpu, D); }             // RRC D
    0x0B => { rrc_r!(cpu, E); }             // RRC E
    0x0C => { rrc_r!(cpu, H); }             // RRC H
    0x0D => { rrc_r!(cpu, L); }             // RRC L
    0x0E => { rrc_mhl!(cpu); }              // RRC (HL)
    0x0F => { rrc_r!(cpu, A); }             // RRC A
    
    0x10 => { rl_r!(cpu, B); }              // RL B
    0x11 => { rl_r!(cpu, C); }              // RL C
    0x12 => { rl_r!(cpu, D); }              // RL D
    0x13 => { rl_r!(cpu, E); }              // RL E
    0x14 => { rl_r!(cpu, H); }              // RL H
    0x15 => { rl_r!(cpu, L); }              // RL L
    0x16 => { rl_mhl!(cpu); }               // RL (HL)
    0x17 => { rl_r!(cpu, A); }              // RL A
    0x18 => { rr_r!(cpu, B); }              // RR B
    0x19 => { rr_r!(cpu, C); }              // RR C
    0x1A => { rr_r!(cpu, D); }              // RR D
    0x1B => { rr_r!(cpu, E); }              // RR E
    0x1C => { rr_r!(cpu, H); }              // RR H
    0x1D => { rr_r!(cpu, L); }              // RR L
    0x1E => { rr_mhl!(cpu); }               // RR (HL)
    0x1F => { rr_r!(cpu, A); }              // RR A

    0x20 => { sla_r!(cpu, B); }             // SLA B
    0x21 => { sla_r!(cpu, C); }             // SLA C
    0x22 => { sla_r!(cpu, D); }             // SLA D
    0x23 => { sla_r!(cpu, E); }             // SLA E
    0x24 => { sla_r!(cpu, H); }             // SLA H
    0x25 => { sla_r!(cpu, L); }             // SLA L
    0x26 => { sla_mhl!(cpu); }              // SLA (HL)
    0x27 => { sla_r!(cpu, A); }             // SLA A
    0x28 => { sra_r!(cpu, B); }             // SRA B
    0x29 => { sra_r!(cpu, C); }             // SRA C
    0x2A => { sra_r!(cpu, D); }             // SRA D
    0x2B => { sra_r!(cpu, E); }             // SRA E
    0x2C => { sra_r!(cpu, H); }             // SRA H
    0x2D => { sra_r!(cpu, L); }             // SRA L
    0x2E => { sra_mhl!(cpu); }              // SRA (HL)
    0x2F => { sra_r!(cpu, A); }             // SRA A

    0x30 => { swap_r!(cpu, B); }            // SWAP B
    0x31 => { swap_r!(cpu, C); }            // SWAP C
    0x32 => { swap_r!(cpu, D); }            // SWAP D
    0x33 => { swap_r!(cpu, E); }            // SWAP E
    0x34 => { swap_r!(cpu, H); }            // SWAP H
    0x35 => { swap_r!(cpu, L); }            // SWAP L
    0x36 => { swap_mhl!(cpu); }             // SWAP (HL)
    0x37 => { swap_r!(cpu, A); }            // SWAP A
    0x38 => { srl_r!(cpu, B); }             // SRL B
    0x39 => { srl_r!(cpu, C); }             // SRL C
    0x3A => { srl_r!(cpu, D); }             // SRL D
    0x3B => { srl_r!(cpu, E); }             // SRL E
    0x3C => { srl_r!(cpu, H); }             // SRL H
    0x3D => { srl_r!(cpu, L); }             // SRL L
    0x3E => { srl_mhl!(cpu); }              // SRL (HL)
    0x3F => { srl_r!(cpu, A); }             // SRL A

    0x40 => { bit_r!(cpu, 0, B); }          // BIT 0,B
    0x41 => { bit_r!(cpu, 0, C); }          // BIT 0,C
    0x42 => { bit_r!(cpu, 0, D); }          // BIT 0,D
    0x43 => { bit_r!(cpu, 0, E); }          // BIT 0,E
    0x44 => { bit_r!(cpu, 0, H); }          // BIT 0,H
    0x45 => { bit_r!(cpu, 0, L); }          // BIT 0,L
    0x46 => { bit_mhl!(cpu, 0); }           // BIT 0,(HL)
    0x47 => { bit_r!(cpu, 0, A); }          // BIT 0,A
    0x48 => { bit_r!(cpu, 1, B); }          // BIT 1,B
    0x49 => { bit_r!(cpu, 1, C); }          // BIT 1,C
    0x4A => { bit_r!(cpu, 1, D); }          // BIT 1,D
    0x4B => { bit_r!(cpu, 1, E); }          // BIT 1,E
    0x4C => { bit_r!(cpu, 1, H); }          // BIT 1,H
    0x4D => { bit_r!(cpu, 1, L); }          // BIT 1,L
    0x4E => { bit_mhl!(cpu, 1); }           // BIT 1,(HL)
    0x4F => { bit_r!(cpu, 1, A); }          // BIT 1,A

    0x50 => { bit_r!(cpu, 2, B); }          // BIT 2,B
    0x51 => { bit_r!(cpu, 2, C); }          // BIT 2,C
    0x52 => { bit_r!(cpu, 2, D); }          // BIT 2,D
    0x53 => { bit_r!(cpu, 2, E); }          // BIT 2,E
    0x54 => { bit_r!(cpu, 2, H); }          // BIT 2,H
    0x55 => { bit_r!(cpu, 2, L); }          // BIT 2,L
    0x56 => { bit_mhl!(cpu, 2); }           // BIT 2,(HL)
    0x57 => { bit_r!(cpu, 2, A); }          // BIT 2,A
    0x58 => { bit_r!(cpu, 3, B); }          // BIT 3,B
    0x59 => { bit_r!(cpu, 3, C); }          // BIT 3,C
    0x5A => { bit_r!(cpu, 3, D); }          // BIT 3,D
    0x5B => { bit_r!(cpu, 3, E); }          // BIT 3,E
    0x5C => { bit_r!(cpu, 3, H); }          // BIT 3,H
    0x5D => { bit_r!(cpu, 3, L); }          // BIT 3,L
    0x5E => { bit_mhl!(cpu, 3); }           // BIT 3,(HL)
    0x5F => { bit_r!(cpu, 3, A); }          // BIT 3,A

    0x60 => { bit_r!(cpu, 4, B); }          // BIT 4,B
    0x61 => { bit_r!(cpu, 4, C); }          // BIT 4,C
    0x62 => { bit_r!(cpu, 4, D); }          // BIT 4,D
    0x63 => { bit_r!(cpu, 4, E); }          // BIT 4,E
    0x64 => { bit_r!(cpu, 4, H); }          // BIT 4,H
    0x65 => { bit_r!(cpu, 4, L); }          // BIT 4,L
    0x66 => { bit_mhl!(cpu, 4); }           // BIT 4,(HL)
    0x67 => { bit_r!(cpu, 4, A); }          // BIT 4,A
    0x68 => { bit_r!(cpu, 5, B); }          // BIT 5,B
    0x69 => { bit_r!(cpu, 5, C); }          // BIT 5,C
    0x6A => { bit_r!(cpu, 5, D); }          // BIT 5,D
    0x6B => { bit_r!(cpu, 5, E); }          // BIT 5,E
    0x6C => { bit_r!(cpu, 5, H); }          // BIT 5,H
    0x6D => { bit_r!(cpu, 5, L); }          // BIT 5,L
    0x6E => { bit_mhl!(cpu, 5); }           // BIT 5,(HL)
    0x6F => { bit_r!(cpu, 5, A); }          // BIT 5,A

    0x70 => { bit_r!(cpu, 6, B); }          // BIT 6,B
    0x71 => { bit_r!(cpu, 6, C); }          // BIT 6,C
    0x72 => { bit_r!(cpu, 6, D); }          // BIT 6,D
    0x73 => { bit_r!(cpu, 6, E); }          // BIT 6,E
    0x74 => { bit_r!(cpu, 6, H); }          // BIT 6,H
    0x75 => { bit_r!(cpu, 6, L); }          // BIT 6,L
    0x76 => { bit_mhl!(cpu, 6); }           // BIT 6,(HL)
    0x77 => { bit_r!(cpu, 6, A); }          // BIT 6,A
    0x78 => { bit_r!(cpu, 7, B); }          // BIT 7,B
    0x79 => { bit_r!(cpu, 7, C); }          // BIT 7,C
    0x7A => { bit_r!(cpu, 7, D); }          // BIT 7,D
    0x7B => { bit_r!(cpu, 7, E); }          // BIT 7,E
    0x7C => { bit_r!(cpu, 7, H); }          // BIT 7,H
    0x7D => { bit_r!(cpu, 7, L); }          // BIT 7,L
    0x7E => { bit_mhl!(cpu, 7); }           // BIT 7,(HL)
    0x7F => { bit_r!(cpu, 7, A); }          // BIT 7,A

    0x80 => { res_r!(cpu, 0, B); }          // RES 0,B
    0x81 => { res_r!(cpu, 0, C); }          // RES 0,C
    0x82 => { res_r!(cpu, 0, D); }          // RES 0,D
    0x83 => { res_r!(cpu, 0, E); }          // RES 0,E
    0x84 => { res_r!(cpu, 0, H); }          // RES 0,H
    0x85 => { res_r!(cpu, 0, L); }          // RES 0,L
    0x86 => { res_mhl!(cpu, 0); }           // RES 0,(HL)
    0x87 => { res_r!(cpu, 0, A); }          // RES 0,A
    0x88 => { res_r!(cpu, 1, B); }          // RES 1,B
    0x89 => { res_r!(cpu, 1, C); }          // RES 1,C
    0x8A => { res_r!(cpu, 1, D); }          // RES 1,D
    0x8B => { res_r!(cpu, 1, E); }          // RES 1,E
    0x8C => { res_r!(cpu, 1, H); }          // RES 1,H
    0x8D => { res_r!(cpu, 1, L); }          // RES 1,L
    0x8E => { res_mhl!(cpu, 1); }           // RES 1,(HL)
    0x8F => { res_r!(cpu, 1, A); }          // RES 1,A

    0x90 => { res_r!(cpu, 2, B); }          // RES 2,B
    0x91 => { res_r!(cpu, 2, C); }          // RES 2,C
    0x92 => { res_r!(cpu, 2, D); }          // RES 2,D
    0x93 => { res_r!(cpu, 2, E); }          // RES 2,E
    0x94 => { res_r!(cpu, 2, H); }          // RES 2,H
    0x95 => { res_r!(cpu, 2, L); }          // RES 2,L
    0x96 => { res_mhl!(cpu, 2); }           // RES 2,(HL)
    0x97 => { res_r!(cpu, 2, A); }          // RES 2,A
    0x98 => { res_r!(cpu, 3, B); }          // RES 3,B
    0x99 => { res_r!(cpu, 3, C); }          // RES 3,C
    0x9A => { res_r!(cpu, 3, D); }          // RES 3,D
    0x9B => { res_r!(cpu, 3, E); }          // RES 3,E
    0x9C => { res_r!(cpu, 3, H); }          // RES 3,H
    0x9D => { res_r!(cpu, 3, L); }          // RES 3,L
    0x9E => { res_mhl!(cpu, 3); }           // RES 3,(HL)
    0x9F => { res_r!(cpu, 3, A); }          // RES 3,A

    0xA0 => { res_r!(cpu, 4, B); }          // RES 4,B
    0xA1 => { res_r!(cpu, 4, C); }          // RES 4,C
    0xA2 => { res_r!(cpu, 4, D); }          // RES 4,D
    0xA3 => { res_r!(cpu, 4, E); }          // RES 4,E
    0xA4 => { res_r!(cpu, 4, H); }          // RES 4,H
    0xA5 => { res_r!(cpu, 4, L); }          // RES 4,L
    0xA6 => { res_mhl!(cpu, 4); }           // RES 4,(HL)
    0xA7 => { res_r!(cpu, 4, A); }          // RES 4,A
    0xA8 => { res_r!(cpu, 5, B); }          // RES 5,B
    0xA9 => { res_r!(cpu, 5, C); }          // RES 5,C
    0xAA => { res_r!(cpu, 5, D); }          // RES 5,D
    0xAB => { res_r!(cpu, 5, E); }          // RES 5,E
    0xAC => { res_r!(cpu, 5, H); }          // RES 5,H
    0xAD => { res_r!(cpu, 5, L); }          // RES 5,L
    0xAE => { res_mhl!(cpu, 5); }           // RES 5,(HL)
    0xAF => { res_r!(cpu, 5, A); }          // RES 5,A

    0xB0 => { res_r!(cpu, 6, B); }          // RES 6,B
    0xB1 => { res_r!(cpu, 6, C); }          // RES 6,C
    0xB2 => { res_r!(cpu, 6, D); }          // RES 6,D
    0xB3 => { res_r!(cpu, 6, E); }          // RES 6,E
    0xB4 => { res_r!(cpu, 6, H); }          // RES 6,H
    0xB5 => { res_r!(cpu, 6, L); }          // RES 6,L
    0xB6 => { res_mhl!(cpu, 6); }           // RES 6,(HL)
    0xB7 => { res_r!(cpu, 6, A); }          // RES 6,A
    0xB8 => { res_r!(cpu, 7, B); }          // RES 7,B
    0xB9 => { res_r!(cpu, 7, C); }          // RES 7,C
    0xBA => { res_r!(cpu, 7, D); }          // RES 7,D
    0xBB => { res_r!(cpu, 7, E); }          // RES 7,E
    0xBC => { res_r!(cpu, 7, H); }          // RES 7,H
    0xBD => { res_r!(cpu, 7, L); }          // RES 7,L
    0xBE => { res_mhl!(cpu, 7); }           // RES 7,(HL)
    0xBF => { res_r!(cpu, 7, A); }          // RES 7,A

    0xC0 => { set_r!(cpu, 0, B); }          // SET 0,B
    0xC1 => { set_r!(cpu, 0, C); }          // SET 0,C
    0xC2 => { set_r!(cpu, 0, D); }          // SET 0,D
    0xC3 => { set_r!(cpu, 0, E); }          // SET 0,E
    0xC4 => { set_r!(cpu, 0, H); }          // SET 0,H
    0xC5 => { set_r!(cpu, 0, L); }          // SET 0,L
    0xC6 => { set_mhl!(cpu, 0); }           // SET 0,(HL)
    0xC7 => { set_r!(cpu, 0, A); }          // SET 0,A
    0xC8 => { set_r!(cpu, 1, B); }          // SET 1,B
    0xC9 => { set_r!(cpu, 1, C); }          // SET 1,C
    0xCA => { set_r!(cpu, 1, D); }          // SET 1,D
    0xCB => { set_r!(cpu, 1, E); }          // SET 1,E
    0xCC => { set_r!(cpu, 1, H); }          // SET 1,H
    0xCD => { set_r!(cpu, 1, L); }          // SET 1,L
    0xCE => { set_mhl!(cpu, 1); }           // SET 1,(HL)
    0xCF => { set_r!(cpu, 1, A); }          // SET 1,A

    0xD0 => { set_r!(cpu, 2, B); }          // SET 2,B
    0xD1 => { set_r!(cpu, 2, C); }          // SET 2,C
    0xD2 => { set_r!(cpu, 2, D); }          // SET 2,D
    0xD3 => { set_r!(cpu, 2, E); }          // SET 2,E
    0xD4 => { set_r!(cpu, 2, H); }          // SET 2,H
    0xD5 => { set_r!(cpu, 2, L); }          // SET 2,L
    0xD6 => { set_mhl!(cpu, 2); }           // SET 2,(HL)
    0xD7 => { set_r!(cpu, 2, A); }          // SET 2,A
    0xD8 => { set_r!(cpu, 3, B); }          // SET 3,B
    0xD9 => { set_r!(cpu, 3, C); }          // SET 3,C
    0xDA => { set_r!(cpu, 3, D); }          // SET 3,D
    0xDB => { set_r!(cpu, 3, E); }          // SET 3,E
    0xDC => { set_r!(cpu, 3, H); }          // SET 3,H
    0xDD => { set_r!(cpu, 3, L); }          // SET 3,L
    0xDE => { set_mhl!(cpu, 3); }           // SET 3,(HL)
    0xDF => { set_r!(cpu, 3, A); }          // SET 3,A

    0xE0 => { set_r!(cpu, 4, B); }          // SET 4,B
    0xE1 => { set_r!(cpu, 4, C); }          // SET 4,C
    0xE2 => { set_r!(cpu, 4, D); }          // SET 4,D
    0xE3 => { set_r!(cpu, 4, E); }          // SET 4,E
    0xE4 => { set_r!(cpu, 4, H); }          // SET 4,H
    0xE5 => { set_r!(cpu, 4, L); }          // SET 4,L
    0xE6 => { set_mhl!(cpu, 4); }           // SET 4,(HL)
    0xE7 => { set_r!(cpu, 4, A); }          // SET 4,A
    0xE8 => { set_r!(cpu, 5, B); }          // SET 5,B
    0xE9 => { set_r!(cpu, 5, C); }          // SET 5,C
    0xEA => { set_r!(cpu, 5, D); }          // SET 5,D
    0xEB => { set_r!(cpu, 5, E); }          // SET 5,E
    0xEC => { set_r!(cpu, 5, H); }          // SET 5,H
    0xED => { set_r!(cpu, 5, L); }          // SET 5,L
    0xEE => { set_mhl!(cpu, 5); }           // SET 5,(HL)
    0xEF => { set_r!(cpu, 5, A); }          // SET 5,A

    0xF0 => { set_r!(cpu, 6, B); }          // SET 6,B
    0xF1 => { set_r!(cpu, 6, C); }          // SET 6,C
    0xF2 => { set_r!(cpu, 6, D); }          // SET 6,D
    0xF3 => { set_r!(cpu, 6, E); }          // SET 6,E
    0xF4 => { set_r!(cpu, 6, H); }          // SET 6,H
    0xF5 => { set_r!(cpu, 6, L); }          // SET 6,L
    0xF6 => { set_mhl!(cpu, 6); }           // SET 6,(HL)
    0xF7 => { set_r!(cpu, 6, A); }          // SET 6,A
    0xF8 => { set_r!(cpu, 7, B); }          // SET 7,B
    0xF9 => { set_r!(cpu, 7, C); }          // SET 7,C
    0xFA => { set_r!(cpu, 7, D); }          // SET 7,D
    0xFB => { set_r!(cpu, 7, E); }          // SET 7,E
    0xFC => { set_r!(cpu, 7, H); }          // SET 7,H
    0xFD => { set_r!(cpu, 7, L); }          // SET 7,L
    0xFE => { set_mhl!(cpu, 7); }           // SET 7,(HL)
    0xFF => { set_r!(cpu, 7, A); }          // SET 7,A
  }
  Ok(())
}
