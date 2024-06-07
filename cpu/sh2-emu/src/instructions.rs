use crate::bus::BusInterface;
use crate::Sh2;

pub fn execute<B: BusInterface>(cpu: &mut Sh2, opcode: u16, bus: &mut B) {
    match opcode {
        0b0000_0000_0001_1001 => todo!("DIV0U"),
        0b0000_0000_0000_1011 => todo!("RTS"),
        0b0000_0000_0000_1000 => todo!("CLRT"),
        0b0000_0000_0010_1000 => todo!("CLRMAC"),
        0b0000_0000_0000_1001 => todo!("NOP"),
        0b0000_0000_0010_1011 => todo!("RTE"),
        0b0000_0000_0001_1000 => todo!("SETT"),
        0b0000_0000_0001_1011 => todo!("SLEEP"),
        _ => execute_xnnx(cpu, opcode, bus),
    }
}

#[inline]
fn execute_xnnx<B: BusInterface>(cpu: &mut Sh2, opcode: u16, bus: &mut B) {
    match opcode & 0b1111_0000_0000_1111 {
        0b0110_0000_0000_0011 => todo!("MOV Rm, Rn"),
        0b0010_0000_0000_0000 => todo!("MOV.B Rm, @Rn"),
        0b0010_0000_0000_0001 => todo!("MOV.W Rm, @Rn"),
        0b0010_0000_0000_0010 => todo!("MOV.L Rm, @Rn"),
        0b0110_0000_0000_0000 => todo!("MOV.B @Rm, Rn"),
        0b0110_0000_0000_0001 => todo!("MOV.W @Rm, Rn"),
        0b0110_0000_0000_0010 => todo!("MOV.L @Rm, Rn"),
        0b0010_0000_0000_0100 => todo!("MOV.B Rm, @-Rn"),
        0b0010_0000_0000_0101 => todo!("MOV.W Rm, @-Rn"),
        0b0010_0000_0000_0110 => todo!("MOV.L Rm, @-Rn"),
        0b0110_0000_0000_0100 => todo!("MOV.B @Rm+, Rn"),
        0b0110_0000_0000_0101 => todo!("MOV.W @Rm+, Rn"),
        0b0110_0000_0000_0110 => todo!("MOV.L @Rm+, Rn"),
        0b0000_0000_0000_0100 => todo!("MOV.B Rm, @(R0,Rn)"),
        0b0000_0000_0000_0101 => todo!("MOV.W Rm, @(R0,Rn)"),
        0b0000_0000_0000_0110 => todo!("MOV.L Rm, @(R0,Rn)"),
        0b0000_0000_0000_1100 => todo!("MOV.B @(R0,Rm), Rn"),
        0b0000_0000_0000_1101 => todo!("MOV.W @(R0,Rm), Rn"),
        0b0000_0000_0000_1110 => todo!("MOV.L @(R0,Rm), Rn"),
        0b0110_0000_0000_1000 => todo!("SWAP.B Rm, Rn"),
        0b0110_0000_0000_1001 => todo!("SWAP.W Rm, Rn"),
        0b0010_0000_0000_1101 => todo!("XTRCT Rm, Rn"),
        0b0011_0000_0000_1100 => todo!("ADD Rm, Rn"),
        0b0011_0000_0000_1110 => todo!("ADDC Rm, Rn"),
        0b0011_0000_0000_1111 => todo!("ADDV Rm, Rn"),
        0b0011_0000_0000_0000 => todo!("CMP/EQ Rm, Rn"),
        0b0011_0000_0000_0010 => todo!("CMP/HS Rm, Rn"),
        0b0011_0000_0000_0011 => todo!("CMP/GE Rm, Rn"),
        0b0011_0000_0000_0110 => todo!("CMP/HI Rm, Rn"),
        0b0011_0000_0000_0111 => todo!("CMP/GT Rm, Rn"),
        0b0010_0000_0000_1100 => todo!("CMP/ST Rm, Rn"),
        0b0011_0000_0000_0100 => todo!("DIV1 Rm, Rn"),
        0b0010_0000_0000_0111 => todo!("DIV0S Rm, Rn"),
        0b0011_0000_0000_1101 => todo!("DMULS.L Rm, Rn"),
        0b0011_0000_0000_0101 => todo!("DMULU.L Rm, Rn"),
        0b0110_0000_0000_1110 => todo!("EXTS.B Rm, Rn"),
        0b0110_0000_0000_1111 => todo!("EXTS.W Rm, Rn"),
        0b0110_0000_0000_1100 => todo!("EXTU.B Rm, Rn"),
        0b0110_0000_0000_1101 => todo!("EXTU.W Rm, Rn"),
        0b0000_0000_0000_1111 => todo!("MAC.L @Rm+, @Rn+"),
        0b0100_0000_0000_1111 => todo!("MAC @Rm+, @Rn+"),
        0b0000_0000_0000_0111 => todo!("MUL.L Rm, Rn"),
        0b0010_0000_0000_1111 => todo!("MULS.W Rm, Rn"),
        0b0010_0000_0000_1110 => todo!("MULU.W Rm, Rn"),
        0b0110_0000_0000_1011 => todo!("NEG Rm, Rn"),
        0b0110_0000_0000_1010 => todo!("NEGC Rm, Rn"),
        0b0011_0000_0000_1000 => todo!("SUB Rm, Rn"),
        0b0011_0000_0000_1010 => todo!("SUBC Rm, Rn"),
        0b0011_0000_0000_1011 => todo!("SUBV Rm, Rn"),
        0b0010_0000_0000_1001 => todo!("AND Rm, Rn"),
        0b0110_0000_0000_0111 => todo!("NOT Rm, Rn"),
        0b0010_0000_0000_1011 => todo!("OR Rm, Rn"),
        0b0010_0000_0000_1000 => todo!("TST Rm, Rn"),
        0b0010_0000_0000_1010 => todo!("XOR Rm, Rn"),
        _ => execute_xxnn(cpu, opcode, bus),
    }
}

#[inline]
fn execute_xxnn<B: BusInterface>(cpu: &mut Sh2, opcode: u16, bus: &mut B) {
    match opcode & 0b1111_1111_0000_0000 {
        0b1000_0000_0000_0000 => todo!("MOV.B R0, @(disp,Rn)"),
        0b1000_0001_0000_0000 => todo!("MOV.W R0, @(disp,Rn)"),
        0b1000_0100_0000_0000 => todo!("MOV.B @(disp,Rm), R0"),
        0b1000_0101_0000_0000 => todo!("MOV.W @(disp,Rm), R0"),
        0b1100_0000_0000_0000 => todo!("MOV.B R0, @(disp,GBR)"),
        0b1100_0001_0000_0000 => todo!("MOV.W R0, @(disp,GBR)"),
        0b1100_0010_0000_0000 => todo!("MOV.L R0, @(disp,GBR)"),
        0b1100_0100_0000_0000 => todo!("MOV.B @(disp,GBR), R0"),
        0b1100_0101_0000_0000 => todo!("MOV.W @(disp,GBR), R0"),
        0b1100_0110_0000_0000 => todo!("MOV.L @(disp,GBR), R0"),
        0b1100_0111_0000_0000 => todo!("MOVA @(disp,PC), R0"),
        0b1000_1000_0000_0000 => todo!("CMP/EQ #imm, R0"),
        0b1100_1001_0000_0000 => todo!("AND #imm, R0"),
        0b1100_1101_0000_0000 => todo!("AND.B #imm, @(R0,GBR)"),
        0b1100_1011_0000_0000 => todo!("OR #imm, R0"),
        0b1100_1111_0000_0000 => todo!("OR.B #imm, @(R0,GBR)"),
        0b1100_1000_0000_0000 => todo!("TST #imm, R0"),
        0b1100_1100_0000_0000 => todo!("TST.B #imm, @(R0,GBR)"),
        0b1100_1010_0000_0000 => todo!("XOR #imm, R0"),
        0b1100_1110_0000_0000 => todo!("XOR.B #imm, @(R0,GBR)"),
        0b1000_1011_0000_0000 => todo!("BF label"),
        0b1000_1111_0000_0000 => todo!("BF/S label"),
        0b1000_1001_0000_0000 => todo!("BT label"),
        0b1000_1101_0000_0000 => todo!("BT/S label"),
        0b1100_0011_0000_0000 => todo!("TRAPA #imm"),
        _ => execute_xnxx(cpu, opcode, bus),
    }
}

#[inline]
fn execute_xnxx<B: BusInterface>(cpu: &mut Sh2, opcode: u16, bus: &mut B) {
    match opcode & 0b1111_0000_1111_1111 {
        0b0000_0000_0010_1001 => todo!("MOVT Rn"),
        0b0100_0000_0001_0001 => todo!("CMP/PZ Rn"),
        0b0100_0000_0001_0101 => todo!("CMP/PL Rn"),
        0b0100_0000_0001_0000 => todo!("DT Rn"),
        0b0100_0000_0001_1011 => todo!("TAS.B @Rn"),
        0b0100_0000_0000_0100 => todo!("ROTL Rn"),
        0b0100_0000_0000_0101 => todo!("ROTR Rn"),
        0b0100_0000_0010_0100 => todo!("ROTCL Rn"),
        0b0100_0000_0010_0101 => todo!("ROTCR Rn"),
        0b0100_0000_0010_0000 => todo!("SHAL Rn"),
        0b0100_0000_0010_0001 => todo!("SHAR Rn"),
        0b0100_0000_0000_0000 => todo!("SHLL Rn"),
        0b0100_0000_0000_0001 => todo!("SHLR Rn"),
        0b0100_0000_0000_1000 => todo!("SHLL2 Rn"),
        0b0100_0000_0000_1001 => todo!("SHLR2 Rn"),
        0b0100_0000_0001_1000 => todo!("SHLL8 Rn"),
        0b0100_0000_0001_1001 => todo!("SHLR8 Rn"),
        0b0100_0000_0010_1000 => todo!("SHLL16 Rn"),
        0b0100_0000_0010_1001 => todo!("SHLR16 Rn"),
        0b0000_0000_0010_0011 => todo!("BRAF Rm"),
        0b0000_0000_0000_0011 => todo!("BSRF Rm"),
        0b0100_0000_0010_1011 => todo!("JMP @Rm"),
        0b0100_0000_0000_1011 => todo!("JSR @Rm"),
        0b0100_0000_0000_1110 => todo!("LDC Rm, SR"),
        0b0100_0000_0001_1110 => todo!("LDC Rm, GBR"),
        0b0100_0000_0010_1110 => todo!("LDC Rm, VBR"),
        0b0100_0000_0000_0111 => todo!("LDC.L @Rm+, SR"),
        0b0100_0000_0001_0111 => todo!("LDC.L @Rm+, GBR"),
        0b0100_0000_0010_0111 => todo!("LDC.L @Rm+, VBR"),
        0b0100_0000_0000_1010 => todo!("LDS Rm, MACH"),
        0b0100_0000_0001_1010 => todo!("LDS Rm, MACL"),
        0b0100_0000_0010_1010 => todo!("LDS Rm, PR"),
        0b0100_0000_0000_0110 => todo!("LDS.L @Rm+, MACH"),
        0b0100_0000_0001_0110 => todo!("LDS.L @Rm+, MACL"),
        0b0100_0000_0010_0110 => todo!("LDS.L @Rm+, PR"),
        0b0000_0000_0000_0010 => todo!("STC SR, Rn"),
        0b0000_0000_0001_0010 => todo!("STC GBR, Rn"),
        0b0000_0000_0010_0010 => todo!("STC VBR, Rn"),
        0b0100_0000_0000_0011 => todo!("STC.L SR, @-Rn"),
        0b0100_0000_0001_0011 => todo!("STC.L GBR, @-Rn"),
        0b0100_0000_0010_0011 => todo!("STC.L VBR, @-Rn"),
        0b0000_0000_0000_1010 => todo!("STS MACH, Rn"),
        0b0000_0000_0001_1010 => todo!("STS MACL, Rn"),
        0b0000_0000_0010_1010 => todo!("STS PR, Rn"),
        0b0100_0000_0000_0010 => todo!("STS.L MACH, @-Rn"),
        0b0100_0000_0001_0010 => todo!("STS.L MACL, @-Rn"),
        0b0100_0000_0010_0010 => todo!("STS.L PR, @-Rn"),
        _ => execute_xnnn(cpu, opcode, bus),
    }
}

#[inline]
fn execute_xnnn<B: BusInterface>(cpu: &mut Sh2, opcode: u16, bus: &mut B) {
    match opcode & 0b1111_0000_0000_0000 {
        0b1110_0000_0000_0000 => todo!("MOV #imm, Rn"),
        0b1001_0000_0000_0000 => todo!("MOV.W @(disp,PC), Rn"),
        0b1101_0000_0000_0000 => todo!("MOV.L @(disp,PC), Rn"),
        0b0001_0000_0000_0000 => todo!("MOV.L Rm, @(disp,Rn)"),
        0b0101_0000_0000_0000 => todo!("MOV.L @(disp,Rm), Rn"),
        0b0111_0000_0000_0000 => todo!("ADD #imm, Rn"),
        0b1010_0000_0000_0000 => todo!("BRA label"),
        0b1011_0000_0000_0000 => todo!("BSR label"),
        _ => todo!("illegal (?) SH-2 opcode {opcode:04X}"),
    }
}
