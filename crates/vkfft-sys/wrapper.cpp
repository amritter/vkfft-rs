#include <vkfft.h>

extern "C"
{
    int vkfft_get_version()
    {
        return VkFFTGetVersion();
    }

    VkFFTResult vkfft_initialize(VkFFTApplication *app, VkFFTConfiguration inputLaunchConfiguration)
    {
        return initializeVkFFT(app, inputLaunchConfiguration);
    }

    void vkfft_delete(VkFFTApplication *app)
    {
        return deleteVkFFT(app);
    }

    VkFFTResult vkfft_sync(VkFFTApplication *app)
    {
        return VkFFTSync(app);
    }

    VkFFTResult vkfft_append(VkFFTApplication *app, int inverse, VkFFTLaunchParams *launchParams)
    {
        return VkFFTAppend(app, inverse, launchParams);
    }

    VkFFTResult vkfft_plan_axis(VkFFTApplication *app, VkFFTPlan *FFTPlan, pfUINT axis_id, pfUINT axis_upload_id, pfUINT inverse, pfUINT reverseBluesteinMultiUpload)
    {
        return VkFFTPlanAxis(app, FFTPlan, axis_id, axis_upload_id, inverse, reverseBluesteinMultiUpload);
    }
}
