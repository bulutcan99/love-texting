use druid::{Data, Lens};

#[derive(Debug, Clone, Data, Lens)]
pub struct AppState {
    content: Content,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            content: Content::Text(StyledText::new(
                "Default text".to_string(),
                FontSize::Fourteen,
                FontWeights::Normal,
            )),
        }
    }
}

#[derive(Debug, Clone, Data, PartialEq)]
pub enum FontSize {
    Twelve,
    Fourteen,
    Sixteen,
    Twenty,
}

impl FontSize {
    pub fn as_u32(&self) -> u32 {
        match self {
            FontSize::Twelve => 12,
            FontSize::Fourteen => 14,
            FontSize::Sixteen => 16,
            FontSize::Twenty => 20,
        }
    }
}

#[derive(Debug, Clone, Data, PartialEq)] // PartialEq eklendi
pub enum FontWeights {
    Normal,
    Bold,
    Bolder,
}

#[derive(Debug, Clone, Data, Lens)]
pub struct StyledText {
    content: String,
    font_size: FontSize,
    font_weight: FontWeights,
}

impl StyledText {
    pub fn new(content: String, font_size: FontSize, font_weight: FontWeights) -> Self {
        Self {
            content,
            font_size,
            font_weight,
        }
    }
}

#[derive(Debug, Clone, Data)]
pub enum Content {
    Header1(StyledText),
    Header2(StyledText),
    Header3(StyledText),
    Text(StyledText),
}

impl Content {
    pub fn as_str(&self) -> &str {
        match self {
            Content::Header1(styled) => &styled.content,
            Content::Header2(styled) => &styled.content,
            Content::Header3(styled) => &styled.content,
            Content::Text(styled) => &styled.content,
        }
    }

    pub fn font_size(&self) -> u32 {
        match self {
            Content::Header1(styled) => styled.font_size.as_u32(),
            Content::Header2(styled) => styled.font_size.as_u32(),
            Content::Header3(styled) => styled.font_size.as_u32(),
            Content::Text(styled) => styled.font_size.as_u32(),
        }
    }

    pub fn font_weight(&self) -> &FontWeights {
        match self {
            Content::Header1(styled) => &styled.font_weight,
            Content::Header2(styled) => &styled.font_weight,
            Content::Header3(styled) => &styled.font_weight,
            Content::Text(styled) => &styled.font_weight,
        }
    }
}
