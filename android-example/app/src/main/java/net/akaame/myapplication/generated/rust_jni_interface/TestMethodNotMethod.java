// Automatically generated by rust_swig
package net.akaame.myapplication.generated.rust_jni_interface;


public final class TestMethodNotMethod {

    public TestMethodNotMethod() {
        mNativeObj = init();
    }
    private static native long init();

    public final void method_not_method() {
        do_method_not_method(mNativeObj);
    }
    private static native void do_method_not_method(long self);

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
    /*package*/ TestMethodNotMethod(InternalPointerMarker marker, long ptr) {
        assert marker == InternalPointerMarker.RAW_PTR;
        this.mNativeObj = ptr;
    }
    /*package*/ long mNativeObj;
}