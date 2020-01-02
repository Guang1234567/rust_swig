package net.akaame.myapplication;

import android.app.Application;
import android.util.Log;

import net.akaame.myapplication.generated.rust_jni_interface.Config;
import net.akaame.myapplication.generated.rust_jni_interface.Session;

/**
 * Created by evgeniy on 16.03.17.
 */

public final class MyApplication extends Application {
    private static MyApplication sSelf;
    private Session mSession;
    private static final String TAG = "exm MyApplication";

    public MyApplication() {
        super();
        sSelf = this;
    }

    @Override
    public void onCreate() {
        Log.i(TAG, "onCreate");
        super.onCreate();
        try {
            System.loadLibrary(BuildConfig.RUST_LIB_NAME);
        } catch (UnsatisfiedLinkError e) {
            Log.e(TAG, "Load libary ERROR: " + e);
            return;
        }
        mSession = new Session(new Config("Android_Config : " + getPackageName()));
    }

    public static MyApplication get() {
        return sSelf;
    }

    public Session getSession() {
        return mSession;
    }
}
