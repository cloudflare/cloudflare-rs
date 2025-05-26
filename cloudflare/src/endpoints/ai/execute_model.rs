use serde::{Deserialize, Serialize};

use crate::framework::endpoint::RequestBody;
use crate::framework::response::ApiSuccess;
use crate::framework::{
    endpoint::{EndpointSpec, Method},
    response::ApiResult,
};

/// Get an inference from a model.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecuteModel<'a> {
    pub account_identifier: &'a str,
    pub model_name: &'a str,
    pub params: ExecuteModelParams,
}

impl EndpointSpec for ExecuteModel<'_> {
    type JsonResponse = ExecuteModelResult;
    type ResponseType = ApiSuccess<Self::JsonResponse>;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "accounts/{}/ai/run/{}",
            self.account_identifier, self.model_name
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        let body = serde_json::to_string(&self.params).unwrap();
        Some(RequestBody::Json(body))
    }
}

/// Represents various inference tasks supported by Workers AI.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecuteModelParams {
    /// Text Classification task.
    ///
    /// Classifies the input text into predefined categories.
    TextClassification {
        /// The text that you want to classify.
        /// Must be at least 1 character long.
        text: String,
    },

    /// Text-to-Image generation task.
    ///
    /// Generates an image based on the provided text description.
    TextToImage(TextToImageParams),

    /// Text-to-Speech generation task.
    ///
    /// Converts text into speech.
    TextToSpeech(TextToSpeechParams),

    /// Text Embedding generation task.
    ///
    /// Converts text into numerical embeddings.
    TextEmbeddings {
        /// The array of texts to embed.
        text: Vec<String>,
    },

    /// Automatic Speech Recognition task.
    ///
    /// Converts audio into text, with optional translation.
    AutomaticSpeechRecognition(AutomaticSpeechRecognitionParams),

    /// Image Classification task.
    ///
    /// Classifies an image into predefined categories.
    ImageClassification {
        /// An array of integers representing the image data (8-bit unsigned integer values).
        image: Vec<u8>,
    },

    /// Object Detection task.
    ///
    /// Detects objects in the input image.
    ObjectDetection {
        /// An array of integers representing the image data (8-bit unsigned integer values).
        image: Vec<u8>,
    },

    /// General Prompt task.
    ///
    /// Generates a response based on the provided input text.
    Prompt(PromptParams),

    /// Messages task.
    ///
    /// Handles conversation-based input and output.
    Messages(MessagesParams),

    /// Translation task.
    /// Translates text into the specified language.
    Translation(TranslationParams),

    /// Summarization task.
    /// Summarizes the provided input text.
    Summarization(SummarizationParams),

    /// Image-to-Text task.
    /// Converts an image into text-based descriptions.
    ImageToText(ImageToTextParams),
}

/// Parameters for the `TextToImage` task.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TextToImageParams {
    /// A text description of the image to generate.
    /// Must be at least 1 character long.
    pub prompt: String,

    /// Controls how closely the generated image should adhere to the prompt.
    pub guidance: Option<f64>,

    /// The height of the generated image in pixels. Must be between 256 and 2048.
    pub height: Option<u32>,

    /// An array of integers representing the image data for img2img tasks.
    pub image: Option<Vec<u8>>,

    /// A base64-encoded string of the input image for img2img tasks.
    pub image_b64: Option<String>,

    /// An array of integers representing mask image data for inpainting.
    pub mask: Option<Vec<u8>>,

    /// Text describing elements to avoid in the generated image.
    pub negative_prompt: Option<String>,

    /// The number of diffusion steps (max 20).
    pub num_steps: Option<u32>,

    /// Random seed for reproducibility.
    pub seed: Option<u64>,

    /// Strength of transformation for img2img tasks (0.0 to 1.0).
    pub strength: Option<f64>,

    /// The width of the generated image in pixels. Must be between 256 and 2048.
    pub width: Option<u32>,
}

/// Parameters for the `TextToSpeech` task.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TextToSpeechParams {
    /// The text to generate speech from.
    /// Must be at least 1 character long.
    pub prompt: String,

    /// The language for the generated speech. Defaults to "en".
    pub lang: Option<String>,
}

/// Parameters for the `AutomaticSpeechRecognition` task.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AutomaticSpeechRecognitionParams {
    /// An array of integers representing the audio data (8-bit unsigned integer values).
    pub audio: Vec<u8>,

    /// The language of the recorded audio.
    pub source_lang: Option<String>,

    /// The target language for translation (currently only English is supported).
    pub target_lang: Option<String>,
}

/// Parameters for the `Prompt` task.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PromptParams {
    /// The input text prompt for the model.
    /// Must be between `1` and `131072` characters long.
    pub prompt: String,

    /// Decreases the likelihood of repeating the same lines verbatim (0 to 2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,

    /// Name of the LoRA (Low-Rank Adaptation) model to fine-tune the base model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lora: Option<String>,

    /// The maximum number of tokens to generate in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Increases the likelihood of introducing new topics (0 to 2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,

    /// If `true`, bypasses chat templates and uses the model's raw format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,

    /// Penalty for repeated tokens (`0` to `2`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f64>,

    /// Random seed for reproducibility (`1` to `9999999999`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,

    /// If `true`, streams the response incrementally using SSE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Controls the randomness of the output (`0` to `5`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// Limits the AI to top 'k' most probable words (`1` to `50`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,

    /// Adjusts creativity of responses (`0` to `2`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
}

/// Parameters for the `Messages` task.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MessagesParams {
    /// The conversation history as an array of message objects.
    pub messages: Vec<Message>,

    /// Decreases the likelihood of repeating the same lines verbatim (`0` to `2`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,

    /// An array of functions or tools available for the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<AssistantFunction>>,

    /// The maximum number of tokens to generate in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Increases the likelihood of introducing new topics (`0` to `2`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,

    /// Penalty for repeated tokens (`0` to `2`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f64>,

    /// Random seed for reproducibility (`1` to `9999999999`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,

    /// If `true`, streams the response incrementally using SSE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Controls the randomness of the output (`0` to `5`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// A list of tools available for the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AssistantTool>>,

    /// Limits the AI to top `k` most probable words (`1` to `50`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,

    /// Adjusts creativity of responses (`0` to `2`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
}

/// Represents a single message in a conversation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    /// The content of the message.
    pub content: String,

    /// The role of the message sender (e.g., "user" or "assistant").
    pub role: MessageRole,
}

impl Message {
    pub fn system(content: String) -> Self {
        Message {
            content,
            role: MessageRole::System,
        }
    }

    pub fn user(content: String) -> Self {
        Message {
            content,
            role: MessageRole::User,
        }
    }

    pub fn assistant(content: String) -> Self {
        Message {
            content,
            role: MessageRole::Assistant,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum MessageRole {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

impl ToString for MessageRole {
    fn to_string(&self) -> String {
        match self {
            MessageRole::System => "System".to_string(),
            MessageRole::User => "User".to_string(),
            MessageRole::Assistant => "Assistant".to_string(),
        }
    }
}

/// Represents a function or tool available for use by the assistant.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssistantFunction {
    /// The function code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// The function name.
    pub name: String,

    /// The function parameters (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
}

/// Represents a tool with additional details.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssistantTool {
    /// A description of the tool.
    pub description: String,

    /// The name of the tool.
    pub name: String,

    /// The parameters associated with the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
}

/// Parameters for the `Translation` task.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TranslationParams {
    /// The target language code (e.g., `"es"` for Spanish).
    pub target_lang: String,

    /// The text to translate. Must be at least 1 character long.
    pub text: String,

    /// The source language code. Defaults to `"en"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_lang: Option<String>,
}

/// Parameters for the `Summarization` task.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SummarizationParams {
    /// The text to summarize. Must be at least 1 character long.
    pub input_text: String,

    /// The maximum length of the generated summary in tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
}

/// Parameters for the `ImageToText` task.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ImageToTextParams {
    /// An array of integers representing the image data.
    pub image: Vec<u8>,

    /// The maximum number of tokens to generate in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// The input text prompt for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// If `true`, bypasses chat templates and uses the model's raw format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,

    /// Controls the randomness of the output; higher values produce more random results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
}

/// Enum representing various AI processing results, including text classification,
/// text-to-image generation, audio generation, and more.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ExecuteModelResult {
    /// Results of text classification, containing an array of classification results.
    TextClassification(Vec<TextClassificationResult>),

    /// The generated image in PNG format.
    TextToImage(String),

    /// The generated audio in MP3 format, base64-encoded.
    Audio(AudioResult),

    /// Text embeddings, containing a nested array of embedding values and their shape.
    TextEmbeddings(TextEmbeddingsResult),

    /// Results of automatic speech recognition.
    AutomaticSpeechRecognition(AutomaticSpeechRecognitionResult),

    /// Results of image classification, containing predicted categories and confidence scores.
    ImageClassification(Vec<ImageClassificationResult>),

    /// Results of object detection within an input image.
    ObjectDetection(Vec<ObjectDetectionResult>),

    /// Generated text response and tool calls from the model.
    ResponseAndToolCallsResult(ResponseAndToolCallsResult),

    /// Results of text translation into a target language.
    Translation(TranslationResult),

    /// Results of text summarization.
    Summarization(SummarizationResult),

    /// Generated description for an input image.
    ImageToText(ImageToTextResult),
}

impl ApiResult for ExecuteModelResult {}

/// Represents a single text classification result.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextClassificationResult {
    /// The classification label assigned to the text (e.g., `'POSITIVE'` or `'NEGATIVE'`).
    pub label: String,

    /// Confidence score indicating the likelihood of the label.
    pub score: f64,
}

/// Represents the generated audio.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct AudioResult {
    /// The generated audio in MP3 format, base64-encoded.
    pub audio: String,
}

/// Represents text embeddings.
///
/// When the `ndarray` feature is enabled, the embeddings are automatically deserialized into an
/// `ndarray::ArrayD<f64>`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TextEmbeddingsResult {
    #[cfg(feature = "ndarray")]
    /// Embeddings of the requested text values.
    pub data: ndarray::ArrayD<f64>,

    #[cfg(not(feature = "ndarray"))]
    /// Embeddings of the requested text values.
    pub data: Vec<serde_json::Value>,

    /// The shape of the embedding array.
    pub shape: Vec<usize>,
}

/// Represents automatic speech recognition results.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AutomaticSpeechRecognitionResult {
    /// The transcription of the audio.
    pub text: String,

    /// The transcription in VTT format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vtt: Option<String>,

    /// The word count of the transcription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_count: Option<usize>,

    /// Array of words with timing information.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub words: Vec<WordTiming>,
}

/// Represents timing information for words in an automatic speech recognition result.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WordTiming {
    /// The start time of the word.
    pub start: f64,

    /// The end time of the word.
    pub end: f64,

    /// The word itself.
    pub word: String,
}

/// Represents a single image classification result.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageClassificationResult {
    /// The predicted category or class for the input image.
    pub label: String,

    /// Confidence score for the classification.
    pub score: f64,
}

/// Represents a single object detection result.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectDetectionResult {
    /// The bounding box around the detected object.
    #[serde(rename = "box")]
    pub bounding_box: BoundingBox,

    /// The class label or name of the detected object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Confidence score for the object detection.
    pub score: f64,
}

/// Represents the bounding box coordinates for an object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BoundingBox {
    /// The minimum x-coordinate.
    pub xmin: f64,

    /// The maximum x-coordinate.
    pub xmax: f64,

    /// The minimum y-coordinate.
    pub ymin: f64,

    /// The maximum y-coordinate.
    pub ymax: f64,
}

/// Represents a generated text response and tool calls from the model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ResponseAndToolCallsResult {
    /// The generated text response.
    pub response: String,

    /// Array of tool call requests made during the response generation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCall>,
    // TODO: Missing `usage` field
}

/// Represents a single tool call request during response generation.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ToolCall {
    /// The name of the tool.
    pub name: String,

    /// The arguments passed to the tool.
    pub arguments: String,
}

/// Represents translation results.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct TranslationResult {
    /// The translated text in the target language.
    pub translated_text: String,
}

/// Represents summarization results.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct SummarizationResult {
    /// The summarized text.
    pub summary: String,
}

/// Represents a generated description for an input image.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ImageToTextResult {
    /// Generated description for an input image.
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This tests the use-case showcased on the website's Workers AI beta.
    #[test]
    fn test_deserialize_response_and_tool_calls_result() {
        let json = r#"
        {"response":"\"A short story\""}
        "#;

        let response: ExecuteModelResult = serde_json::from_str(json).unwrap();
        assert!(matches!(
            response,
            ExecuteModelResult::ResponseAndToolCallsResult(_)
        ));
    }
}
