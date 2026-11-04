import cn.wps.moffice.pdf.core.io.AtomicFileCommit;

import java.io.File;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;

public final class AtomicFileCommitTest {
    static int pass, fail;
    static void check(boolean value, String name) {
        if (value) pass++; else { fail++; System.out.println("FAIL: " + name); }
    }
    static void put(File file, String value) throws Exception {
        Files.write(file.toPath(), value.getBytes(StandardCharsets.UTF_8));
    }
    static String get(File file) throws Exception {
        return new String(Files.readAllBytes(file.toPath()), StandardCharsets.UTF_8);
    }
    static final class Ops implements AtomicFileCommit.FileOps {
        boolean failBackup, failRecovery, corruptThenFail;
        public boolean exists(File f) { return f.exists(); }
        public long length(File f) { return f.length(); }
        public void copy(File from, File to) throws IOException {
            boolean toBackup = to.getName().contains(".rebuild-backup-");
            boolean fromBackup = from.getName().contains(".rebuild-backup-");
            if (failBackup && toBackup)
                throw new IOException("injected backup failure");
            if (failRecovery && fromBackup)
                throw new IOException("injected recovery failure");
            Files.copy(from.toPath(), to.toPath(), StandardCopyOption.REPLACE_EXISTING);
        }
        public void replace(File from, File to) throws IOException {
            if (corruptThenFail) {
                Files.write(to.toPath(), new byte[] { 0 });
                throw new IOException("injected replace failure");
            }
            Files.move(from.toPath(), to.toPath(), StandardCopyOption.REPLACE_EXISTING);
        }
        public void delete(File f) {
            if (f != null) try { Files.deleteIfExists(f.toPath()); } catch (IOException ignored) { }
        }
    }
    static File[] backups(File dir) {
        File[] files = dir.listFiles((parent, name) -> name.contains(".rebuild-backup-"));
        return files == null ? new File[0] : files;
    }
    public static void main(String[] args) throws Exception {
        File dir = new File(args[0]);
        check(dir.isDirectory() || dir.mkdirs(), "test directory");

        File target = new File(dir, "atomic-target.pdf");
        File temp = new File(dir, "atomic-temp.pdf");
        put(target, "OLD"); put(temp, "NEW");
        check(AtomicFileCommit.commit(temp, target), "replace existing target");
        check("NEW".equals(get(target)), "new bytes committed");
        check(!temp.exists(), "temp consumed");
        check(backups(dir).length == 0, "backup removed");

        File newTarget = new File(dir, "new-target.pdf");
        File newTemp = new File(dir, "new-temp.pdf");
        put(newTemp, "FIRST");
        check(AtomicFileCommit.commit(newTemp, newTarget), "create absent target");
        check("FIRST".equals(get(newTarget)), "absent target bytes");

        put(target, "SAFE"); put(temp, "BROKEN");
        Ops replaceFailure = new Ops(); replaceFailure.corruptThenFail = true;
        check(!AtomicFileCommit.commit(temp, target, replaceFailure), "replace failure reported");
        check("SAFE".equals(get(target)), "replace failure restores target");
        check(backups(dir).length == 0, "recovery backup removed");

        put(target, "SAFE2"); put(temp, "NEW2");
        Ops backupFailure = new Ops(); backupFailure.failBackup = true;
        check(!AtomicFileCommit.commit(temp, target, backupFailure), "backup failure reported");
        check("SAFE2".equals(get(target)), "backup failure leaves target");

        put(target, "LAST-GOOD"); put(temp, "NEW3");
        Ops recoveryFailure = new Ops();
        recoveryFailure.corruptThenFail = true; recoveryFailure.failRecovery = true;
        check(!AtomicFileCommit.commit(temp, target, recoveryFailure),
                "recovery failure reported");
        File[] retained = backups(dir);
        check(retained.length == 1 && "LAST-GOOD".equals(get(retained[0])),
                "failed recovery retains last-good backup");
        retained[0].delete();
        put(target, "SAFE2");

        File empty = new File(dir, "empty.pdf"); put(empty, "");
        check(!AtomicFileCommit.commit(empty, target), "empty temp rejected");
        check("SAFE2".equals(get(target)), "empty temp leaves target");

        System.out.println("ATOMIC_FILE_COMMIT PASS=" + pass + " FAIL=" + fail);
        if (fail != 0) System.exit(1);
    }
}
