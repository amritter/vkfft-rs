#include <vkFFT.h>

int vkfft_get_version();

VkFFTResult vkfft_initialize(VkFFTApplication *app, VkFFTConfiguration inputLaunchConfiguration);

void vkfft_delete(VkFFTApplication *app);

VkFFTResult vkfft_sync(VkFFTApplication *app);

VkFFTResult vkfft_append(VkFFTApplication *app, int inverse, VkFFTLaunchParams *launchParams);

VkFFTResult vkfft_plan_axis(VkFFTApplication *app, VkFFTPlan *FFTPlan, pfUINT axis_id, pfUINT axis_upload_id, pfUINT inverse, pfUINT reverseBluesteinMultiUpload);
