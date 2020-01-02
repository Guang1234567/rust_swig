// Automatically generated by rust_swig
package net.akaame.myapplication.generated.rust_jni_interface;


public final class Interface {

    public Interface() {
        mNativeObj = init();
    }
    private static native long init();

    public final int f(int a0) {
        int ret = do_f(mNativeObj, a0);

        return ret;
    }
    private static native int do_f(long self, int a0);

    public final void set(int x) {
        do_set(mNativeObj, x);
    }
    private static native void do_set(long self, int x);

    public synchronized void delete() {
        if (mNativeObj != 0) {
            do_delete(mNativeObj);
            mNativeObj = 0;
       }
    }
    @Override
    protected void finalize() throws Throwable {
        try {
            delete();
        }
        finally {
             super.finalize();
        }
    }
    private static native void do_delete(long me);
    /*package*/ Interface(InternalPointerMarker marker, long ptr) {
        assert marker == InternalPointerMarker.RAW_PTR;
        this.mNativeObj = ptr;
    }
    /*package*/ long mNativeObj;
}