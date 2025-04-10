pub struct Packet<Pre: Preamble, P: Pdu, Post: Postamble> {
    pub preamble: Pre,
    pub block: PduBlock<P>,
    pub postamble: Post,
}

impl<
    Pre: Preamble<DecodeError = Root::DecodeError>,
    Root: Pdu,
    Post: Postamble<DecodeError = Root::DecodeError>,
> Pdu for Packet<Pre, Root, Post>
{
    type DecodeError = Root::DecodeError;

    fn encode(&self) -> impl Into<Vec<u8>> {
        let mut buffer = Vec::new();
        buffer.extend(self.preamble.encode().into());
        buffer.extend(self.block.encode());
        buffer.extend(self.postamble.encode().into());
        buffer
    }

    fn decode(data: &[u8]) -> Result<Self, Self::DecodeError> {
        let preamble = Pre::decode(&data[..Pre::SIZE])?;
        let block = PduBlock::decode(&data[Pre::SIZE..])?;
        let postamble = Post::decode(&data[Pre::SIZE + block.size()..])?;
        Ok(Packet { preamble, block, postamble })
    }

    fn size(&self) -> usize {
        Pre::SIZE + self.block.size() + self.postamble.size()
    }
}

impl<Pre: Preamble, P: Pdu, Post: Postamble> Packet<Pre, P, Post> {
    pub fn new(preamble: Pre, block: PduBlock<P>, postamble: Post) -> Self {
        Packet { preamble, block, postamble }
    }
}

pub struct PduBlock<P: Pdu>(Vec<P>);

impl<P: Pdu> PduBlock<P> {
    pub fn new(pdus: Vec<P>) -> Self {
        Self(pdus)
    }

    pub fn pdus(&self) -> &[P] {
        &self.0
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        for pdu in &self.0 {
            buffer.extend(pdu.encode().into());
        }
        buffer
    }

    pub fn decode(data: &[u8]) -> Result<Self, P::DecodeError> {
        let mut pdus = Vec::new();
        let mut offset = 0;
        while offset < data.len() {
            let pdu = P::decode(&data[offset..])?;
            offset += pdu.size();
            pdus.push(pdu);
        }
        Ok(PduBlock(pdus))
    }

    pub fn size(&self) -> usize {
        self.0.iter().map(|pdu| pdu.size()).sum()
    }
}

pub trait Pdu {
    type DecodeError: std::error::Error;

    fn encode(&self) -> impl Into<Vec<u8>>;

    fn decode(data: &[u8]) -> Result<Self, Self::DecodeError>
    where
        Self: Sized;

    fn size(&self) -> usize;
}

pub trait Preamble {
    type DecodeError: std::error::Error;

    const SIZE: usize;

    fn encode(&self) -> impl Into<Vec<u8>>;

    fn decode(data: &[u8]) -> Result<Self, Self::DecodeError>
    where
        Self: Sized;
}

pub trait Postamble {
    type DecodeError: std::error::Error;

    fn encode(&self) -> impl Into<Vec<u8>>;

    fn decode(data: &[u8]) -> Result<Self, Self::DecodeError>
    where
        Self: Sized;

    fn size(&self) -> usize;
}
