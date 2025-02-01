use std::sync::Arc;
use std::time::SystemTime;
use vulkano::buffer::Subbuffer;
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::descriptor_set::DescriptorSet;
use vulkano::device::{Device, Queue};
use vulkano::instance::Instance;
use vulkano::pipeline::{ComputePipeline, GraphicsPipeline};
use vulkano::render_pass::Framebuffer;
use vulkano::swapchain::Swapchain;
use vulkano::sync::GpuFuture;

struct App {
    instance: Arc<Instance>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    vertex_buffer: Subbuffer<[MyVertex]>,
    compute_pipeline: Arc<ComputePipeline>,
    descriptor_set: Arc<dyn DescriptorSet>,
    rcx: Option<RenderContext>,
}

struct RenderContext {
    // window: Arc<Window>,
    swapchain: Arc<Swapchain>,
    framebuffers: Vec<Arc<Framebuffer>>,
    pipeline: Arc<GraphicsPipeline>,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    start_time: SystemTime,
    last_frame_time: SystemTime,
}