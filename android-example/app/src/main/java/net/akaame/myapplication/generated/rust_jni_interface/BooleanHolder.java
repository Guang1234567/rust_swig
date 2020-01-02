// Automatically generated by rust_swig
package net.akaame.myapplication.generated.rust_jni_interface;


public final class BooleanHolder {

    public BooleanHolder() {
        mNativeObj = init();
    }
    private static native long init();

    public final void set(boolean v) {
        do_set(mNativeObj, v);
    }
    private static native void do_set(long self, boolean v);

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
    /*package*/ BooleanHolder(InternalPointerMarker marker, long ptr) {
        assert marker == InternalPointerMarker.RAW_PTR;
        this.mNativeObj = ptr;
    }
    /*package*/ long mNativeObj;
}