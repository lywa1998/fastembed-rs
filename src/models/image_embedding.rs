use super::model_info::ModelInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageEmbeddingModel {
    /// Qdrant/clip-ViT-B-32-vision
    ClipVitB32,
    /// Qdrant/resnet50-onnx
    Resnet50,
    /// Qdrant/Unicom-ViT-B-16
    UnicomVitB16,
    /// Qdrant/Unicom-ViT-B-32
    UnicomVitB32,
}

pub fn models_list() -> Vec<ModelInfo<ImageEmbeddingModel>> {
    let models_list = vec![
        ModelInfo {
            model: ImageEmbeddingModel::ClipVitB32,
            dim: 512,
            description: String::from("CLIP vision encoder based on ViT-B/32"),
            model_code: String::from("Qdrant/clip-ViT-B-32-vision"),
            model_file: String::from("model.onnx"),
        },
        ModelInfo {
            model: ImageEmbeddingModel::Resnet50,
            dim: 2048,
            description: String::from("ResNet-50 from `Deep Residual Learning for Image Recognition <https://arxiv.org/abs/1512.03385>`__."),
            model_code: String::from("Qdrant/resnet50-onnx"),
            model_file: String::from("model.onnx"),
        },
        ModelInfo {
            model: ImageEmbeddingModel::UnicomVitB16,
            dim: 768,
            description: String::from("Unicom Unicom-ViT-B-16 from open-metric-learning"),
            model_code: String::from("Qdrant/Unicom-ViT-B-16"),
            model_file: String::from("model.onnx"),
        },
        ModelInfo {
            model: ImageEmbeddingModel::UnicomVitB32,
            dim: 512,
            description: String::from("Unicom Unicom-ViT-B-32 from open-metric-learning"),
            model_code: String::from("Qdrant/Unicom-ViT-B-32"),
            model_file: String::from("model.onnx"),
        }
    ];

    // TODO: Use when out in stable
    // assert_eq!(
    //     std::mem::variant_count::<ImageEmbeddingModel>(),
    //     models_list.len(),
    //     "models::models() is not exhaustive"
    // );

    models_list
}
