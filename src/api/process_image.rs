use leptos::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[cfg(feature = "ssr")]
use std::path::{Path, PathBuf};
#[cfg(feature = "ssr")]
use std::process::Output;
#[cfg(feature = "ssr")]
use thiserror::Error;

#[derive(Deserialize)]
#[cfg_attr(feature = "ssr", derive(serde::Serialize))]
pub enum Layout {
    Rows,
    Columns,
    Both,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[cfg_attr(feature = "ssr", derive(serde::Serialize))]
pub struct TableData {
    data: HashMap<String, Vec<f64>>,
    /// Units are a string value and seperate from the calculation
    /// Not unit conversion is performed during calculations
    /// Defaults to "units"
    units: String,
    /// Layout of the table
    layout: Layout,
    /// precision to use when calculating
    precision: usize,
}

impl TableData {
    fn new(layout: Layout) -> Self {
        Self {
            data: HashMap::new(),
            units: String::from("units"),
            layout,
            precision: 3,
        }
    }

    fn calculation_precision(mut self, precision: usize) -> Self {
        self.precision = precision;
        self
    }

    fn with_units(mut self, units: String) -> Self {
        self.units = units;
        self
    }

    fn get_mut_data(&mut self) -> &mut HashMap<String, Vec<f64>> {
        &mut self.data
    }

    #[cfg(feature = "ssr")]
    async fn calculate_stddev(&self, key: String) -> f64 {
        0.0
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, Error)]
pub enum ImageProcessorError {
    #[error("Image `{0}` doesn't exist on this server")]
    ImageNotFound(String),
    #[error("Invalid path")]
    InvalidPath,
    #[error("Output of ImageProcessor is set to `None`")]
    NoOutput,
}

#[cfg(feature = "ssr")]
pub struct ImageProcessor {
    image_path: PathBuf,
    output: Option<Output>,
    layout: Layout,
}

#[cfg(feature = "ssr")]
impl ImageProcessor {
    fn new(image_path: PathBuf) -> Self {
        Self {
            image_path,
            layout: Layout::Columns,
            output: None,
        }
    }

    fn with_layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    fn set_image_path(&mut self, image_path: PathBuf) {
        self.image_path = image_path;
    }

    async fn extract_data(mut self) -> Result<Self, ImageProcessorError> {
        crate::core::utils::validate_path(&self.image_path)
            .map_err(|_| ImageProcessorError::InvalidPath)?;

        self.output = None;
        Ok(self)
    }
}

#[cfg(feature = "ssr")]
impl TryInto<TableData> for ImageProcessor {
    type Error = ImageProcessorError;

    fn try_into(self) -> Result<TableData, Self::Error> {
        if let Some(data) = self.output {
            // TODO: finish type conversion
            Ok(TableData::new(self.layout))
        } else {
            Err(Self::Error::NoOutput)
        }
    }
}

/// use these errors internally
#[cfg(feature = "ssr")]
#[derive(Debug, Error)]
enum EncodeAsWebPError {
    #[error("File path is missing stem")]
    MissingStem,
    #[error("Invalid file path")]
    InvalidPath,
}

#[cfg(feature = "ssr")]
pub async fn encode_as_webp(file_path: PathBuf) -> Result<(), ServerFnError> {
    crate::core::utils::validate_path(&file_path).map_err(|_| EncodeAsWebPError::InvalidPath)?;

    let stem = file_path
        .file_stem()
        .ok_or_else(|| EncodeAsWebPError::MissingStem)?;

    let parent = file_path.parent();

    let new_path = PathBuf::new()
        .join(parent.unwrap_or_else(|| Path::new("")))
        .join(stem)
        .with_extension("webp");

    tracing::info!(
        "re-writing {} to {}",
        file_path.display(),
        new_path.display()
    );

    Ok(())
}
