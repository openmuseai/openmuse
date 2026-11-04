// Java: import java.util.ArrayList
// TODO_RUST_CODE use ???; // java.util.ArrayList
// Java: import org.apache.poi.util.LittleEndian
use util::org::apache::poi::util::LittleEndian;

// Java: public class SprmUtils
pub struct SprmUtils {
}

// Java: public class SprmUtils
impl SprmUtils {
    // Java: SprmUtils()
    pub fn SprmUtils(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: static byte[] shortArrayToByteArray(short[] convert)
    pub fn shortArrayToByteArray(&self, short[] convert) -> byte[] {
      // Java: byte[] buf = new byte[convert.length * 2];
      // TODO_RUST_CODE
      // Java: for(int x = 0; x < convert.length; ++x)
        // TODO_RUST_CODE
      {
         // Java: LittleEndian.putShort(buf, x * 2, convert[x]);
         // TODO_RUST_CODE
      }
      // Java: return buf;
      // TODO_RUST_CODE
    }

    // Java: static int addSpecialSprm(short instruction, byte[] varParam, ArrayList<byte[]> list)
    pub fn addSpecialSprm(&self, short instruction, byte[] varParam, ArrayList<byte[]> list) -> i32 {
      // Java: byte[] sprm = new byte[varParam.length + 4];
      // TODO_RUST_CODE
      // Java: System.arraycopy(varParam, 0, sprm, 4, varParam.length);
      // TODO_RUST_CODE
      // Java: LittleEndian.putShort(sprm, instruction);
      // TODO_RUST_CODE
      // Java: LittleEndian.putShort(sprm, 2, (short)(varParam.length + 1));
      // TODO_RUST_CODE
      // Java: list.add(sprm);
      // TODO_RUST_CODE
      // Java: return sprm.length;
      // TODO_RUST_CODE
    }

    // Java: static int addSprm(short instruction, byte[] varParam, ArrayList<byte[]> list)
    pub fn addSprm(&self, short instruction, byte[] varParam, ArrayList<byte[]> list) -> i32 {
      // Java: byte[] sprm = new byte[varParam.length + 2];
      // TODO_RUST_CODE
      // Java: System.arraycopy(varParam, 0, sprm, 2, varParam.length);
      // TODO_RUST_CODE
      // Java: LittleEndian.putShort(sprm, instruction);
      // TODO_RUST_CODE
      // Java: list.add(sprm);
      // TODO_RUST_CODE
      // Java: return sprm.length;
      // TODO_RUST_CODE
    }

    // Java: static int addSprm(short instruction, int param, byte[] varParam, ArrayList<byte[]> list)
    pub fn addSprm(&self, short instruction, int param, byte[] varParam, ArrayList<byte[]> list) -> i32 {
      // Java: int type = (instruction & '\ue000') >> 13;
      // TODO_RUST_CODE
      // Java: byte[] sprm = null;
      // TODO_RUST_CODE
      // Java: switch (type)
        // TODO_RUST_CODE
      {
         // Java: case 0:
         // TODO_RUST_CODE
         // Java: case 1:
         // TODO_RUST_CODE
            // Java: sprm = new byte[]{0, 0, (byte)param};
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 2:
         // TODO_RUST_CODE
            // Java: sprm = new byte[4];
            // TODO_RUST_CODE
            // Java: LittleEndian.putShort(sprm, 2, (short)param);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 3:
         // TODO_RUST_CODE
            // Java: sprm = new byte[6];
            // TODO_RUST_CODE
            // Java: LittleEndian.putInt(sprm, 2, param);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 4:
         // TODO_RUST_CODE
         // Java: case 5:
         // TODO_RUST_CODE
            // Java: sprm = new byte[4];
            // TODO_RUST_CODE
            // Java: LittleEndian.putShort(sprm, 2, (short)param);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 6:
         // TODO_RUST_CODE
            // Java: sprm = new byte[3 + varParam.length];
            // TODO_RUST_CODE
            // Java: sprm[2] = (byte)varParam.length;
            // TODO_RUST_CODE
            // Java: System.arraycopy(varParam, 0, sprm, 3, varParam.length);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 7:
         // TODO_RUST_CODE
            // Java: sprm = new byte[5];
            // TODO_RUST_CODE
            // Java: byte[] temp = new byte[4];
            // TODO_RUST_CODE
            // Java: LittleEndian.putInt(temp, 0, param);
            // TODO_RUST_CODE
            // Java: System.arraycopy(temp, 0, sprm, 2, 3);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: default:
         // TODO_RUST_CODE
            // Java: sprm = new byte[0];
            // TODO_RUST_CODE
      }
      // Java: LittleEndian.putShort(sprm, 0, instruction);
      // TODO_RUST_CODE
      // Java: list.add(sprm);
      // TODO_RUST_CODE
      // Java: return sprm.length;
      // TODO_RUST_CODE
    }

    // Java: static void getGrpprl(ArrayList<byte[]> sprmList, byte[] grpprl, int offset)
    pub fn getGrpprl(&self, ArrayList<byte[]> sprmList, byte[] grpprl, int offset) {
      // Java: int index = offset;
      // TODO_RUST_CODE
      // Java: int n = sprmList.size();
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < n; ++i)
        // TODO_RUST_CODE
      {
         // Java: byte[] sprm = (byte[])sprmList.get(i);
         // TODO_RUST_CODE
         // Java: System.arraycopy(sprm, 0, grpprl, index, sprm.length);
         // TODO_RUST_CODE
         // Java: index += sprm.length;
         // TODO_RUST_CODE
      }
    }

    // Java: static byte[] getGrpprl(ArrayList<byte[]> sprmList, int size)
    pub fn getGrpprl(&self, ArrayList<byte[]> sprmList, int size) -> byte[] {
      // Java: byte[] p = new byte[size];
      // TODO_RUST_CODE
      // Java: getGrpprl(sprmList, p, 0);
      // TODO_RUST_CODE
      // Java: return p;
      // TODO_RUST_CODE
    }

    // Java: static int convertBrcToInt(short[] brc)
    pub fn convertBrcToInt(&self, short[] brc) -> i32 {
      // Java: byte[] buf = new byte[4];
      // TODO_RUST_CODE
      // Java: LittleEndian.putShort(buf, brc[0]);
      // TODO_RUST_CODE
      // Java: LittleEndian.putShort(buf, 2, brc[1]);
      // TODO_RUST_CODE
      // Java: return LittleEndian.getInt(buf);
      // TODO_RUST_CODE
    }

}
