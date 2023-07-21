#![cfg_attr(nightly, feature(doc_cfg))]
#![ doc = include_str!("../README.md") ]
#![doc(html_root_url = "https://docs.rs/ws_stream_tungstenite")]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![allow(clippy::suspicious_else_formatting)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_qualifications,
    single_use_lifetimes,
    unreachable_pub,
    variant_size_differences
)]

mod ws_err;
mod ws_event;
mod ws_stream;

pub(crate) mod tung_websocket;

pub use {self::ws_err::WsErr, self::ws_event::WsEvent, self::ws_stream::WsStream};

mod import {
    pub(crate) use {
        async_io_stream::IoStream,
        axum::{
            extract::ws::{
                CloseCode as AxumCloseCode,
                CloseFrame as AxumCloseFrame,
                Message as AxumMessage,
                WebSocket as AxumTungSocket,
            },
            Error as AxumErr,
        },
        // async_tungstenite::WebSocketStream as ATungSocket,
        tokio_tungstenite::WebSocketStream as ATungSocket,
        tokio_util::compat::FuturesAsyncReadCompatExt,
        bitflags::bitflags,
        futures_core::{ready, Stream},
        futures_io::{AsyncBufRead, AsyncRead, AsyncWrite},
        futures_sink::Sink,
        futures_util::FutureExt,
        log::error,
        pharos::{Observable, Observe, ObserveConfig, PharErr, Pharos},
        std::{
            borrow::Cow,
            fmt, io,
            io::{IoSlice, IoSliceMut},
            pin::Pin,
        },
        std::{
            collections::VecDeque,
            sync::Arc,
            task::{Context, Poll},
        },
        tungstenite::{
            // protocol::{frame::coding::CloseCode, CloseFrame},
            Error as TungErr,
            // Message as TungMessage,
        },
    };

    #[cfg(feature = "tokio")]
    //
    pub(crate) use tokio::io::{AsyncRead as TokAsyncRead, AsyncWrite as TokAsyncWrite};

    #[cfg(test)]
    //
    pub(crate) use {
        assert_matches::assert_matches,
        futures::future::join,
        futures::{executor::block_on, SinkExt, StreamExt},
        futures_ringbuf::Endpoint,
        futures_test::task::noop_waker,
        log::*,
        pharos::Channel,
        tungstenite::protocol::Role,
    };
}
