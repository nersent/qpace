from typing import Optional, TypedDict, Union
from google.protobuf.empty_pb2 import Empty
from datetime import timezone
import grpc
import qpace_core as qp
import proto.ohlcv_pb2 as ohlcv_api
import proto.sym_pb2 as sym_api

ENV_REST_ENDPOINT = "QPACE_API_BASE"
ENV_GRPC_ENDPOINT = "QPACE_GRPC_API_BASE"
ENV_API_KEY = "QPACE_API_KEY"
ENV_TELEMETRY = "QPACE_TELEMETRY"

DEFAULT_REST_ENDPOINT = "https://api.qpace.dev/v1"
DEFAULT_GRPC_ENDPOINT = "https://api.qpace.dev/grpc"


# export const protoToSym = (proto: symApi.Sym): qp.Sym => {
#   const sym = new qp.Sym();
#   sym.id = proto.getId()!;
#   if (proto.hasTickerId()) sym.tickerId = proto.getTickerId()!;
#   if (proto.hasPrefix()) sym.prefix = proto.getPrefix()!;
#   if (proto.hasCurrency()) sym.currency = proto.getCurrency()!;
#   if (proto.hasBaseCurrency()) sym.baseCurrency = proto.getBaseCurrency()!;
#   if (proto.hasTicker()) sym.ticker = proto.getTicker()!;
#   if (proto.hasCountry()) sym.country = proto.getCountry()!;
#   if (proto.hasMinTick()) sym.minTick = proto.getMinTick()!;
#   if (proto.hasMinQty()) sym.minQty = proto.getMinQty()!;
#   if (proto.hasPriceScale()) sym.priceScale = proto.getPriceScale()!;
#   if (proto.hasPointValue()) sym.pointValue = proto.getPointValue()!;
#   sym.icons.push(...proto.getIconsList().map(protoToSymIcon));
#   return sym;
# };

# export const symToProto = (sym: qp.Sym): symApi.Sym => {
#   const proto = new symApi.Sym();
#   if (sym.id == null) throw new Error("symbol id is required");
#   proto.setId(sym.id);
#   if (sym.tickerId != null) proto.setTickerId(sym.tickerId);
#   if (sym.prefix != null) proto.setPrefix(sym.prefix);
#   if (sym.currency != null) proto.setCurrency(sym.currency);
#   if (sym.baseCurrency != null) proto.setBaseCurrency(sym.baseCurrency);
#   if (sym.ticker != null) proto.setTicker(sym.ticker);
#   if (sym.country != null) proto.setCountry(sym.country);
#   if (sym.minTick != null) proto.setMinTick(sym.minTick);
#   if (sym.minQty != null) proto.setMinQty(sym.minQty);
#   if (sym.priceScale != null) proto.setPriceScale(sym.priceScale);
#   if (sym.pointValue != null) proto.setPointValue(sym.pointValue);
#   proto.setIconsList(sym.icons.map(symIconToProto));
#   return proto;
# };


# def sym_to_proto(sym: qp.Sym) -> sym_api.Sym:
#     return sym_api.Sym(
#         id=sym.id,
#         ticker_id=sym.ticker_id,
#         prefix=sym.prefix,
#         currency=sym.currency,
#         base_currency=sym.base_currency,
#         ticker=sym.ticker,
#         country=sym.country,
#         min_tick=sym.min_tick,
#         min_qty=sym.min_qty,
#         price_scale=sym.price_scale,
#         point_value=sym.point_value,
#         icons=[icon for icon in sym.icons],
#     )


class SymFilter(TypedDict):
    id: Optional[str]
    ticker_id: Optional[str]
    timeframe: Optional[qp.Timeframe]


class SymQuery(SymFilter):
    limit: Optional[int]
    offset: Optional[int]


def sym_filter_to_proto(filter: SymFilter) -> sym_api.Filter:
    timeframe = filter.get("timeframe", None)
    if timeframe is not None:
        timeframe = str(timeframe)
    proto = sym_api.Filter(
        id=filter["id"],
        ticker_id_pat=filter["ticker_id"],
        timeframe=timeframe,
    )
    return proto


def sym_query_to_proto(query: SymQuery) -> sym_api.Query:
    proto = sym_api.Query(
        filter=sym_filter_to_proto(query), limit=query["limit"], offset=query["offset"]
    )
    return proto


def proto_to_sym(proto: sym_api.Sym) -> qp.Sym:
    sym = qp.Sym()
    sym.id = proto.id
    sym.ticker_id = proto.ticker_id
    sym.prefix = proto.prefix
    sym.currency = proto.currency
    sym.base_currency = proto.base_currency
    sym.ticker = proto.ticker
    sym.country = proto.country
    sym.min_tick = proto.min_tick
    sym.min_qty = proto.min_qty
    sym.price_scale = proto.price_scale
    sym.point_value = proto.point_value
    sym.icons = [proto_to_sym_icon(icon) for icon in proto.icons]
    return sym


def proto_to_sym_icon(proto: sym_api.Icon) -> qp.SymIcon:
    icon = qp.SymIcon()
    icon.url = proto.url
    icon.mime_type = proto.mime_type
    return icon


def get_sym_id(sym: Union[str, qp.Sym]) -> str:
    if isinstance(sym, str):
        return sym
    if isinstance(sym, qp.Sym) and sym.id is not None:
        return sym.id
    raise ValueError(f"Cannot get symbol id from {sym}")


class OhlcvFilter(TypedDict):
    sym: Union[str, qp.Sym]
    timeframe: Optional[qp.Timeframe]


def ohlcv_filter_to_proto(filter: OhlcvFilter) -> ohlcv_api.Filter:
    timeframe = filter.get("timeframe", None)
    if timeframe is not None:
        timeframe = str(timeframe)
    proto = ohlcv_api.Filter(
        sym_id=get_sym_id(filter["sym"]),
        timeframe=timeframe,
    )
    return proto


def proto_to_ohlcv_bar(proto: ohlcv_api.OhlcvBar) -> qp.OhlcvBar:
    open_time = proto.open_time.ToDatetime(tzinfo=timezone.utc)
    close_time = proto.close_time.ToDatetime(tzinfo=timezone.utc)
    return qp.OhlcvBar(
        open_time=open_time,
        close_time=close_time,
        open=proto.open,
        high=proto.high,
        low=proto.low,
        close=proto.close,
        volume=proto.volume,
    )


# def timeframe_to_proto(timeframe: qp.Timeframe) -> sym_api.Timeframe:
#     if timeframe.unknown:
#         return sym_api.Timeframe(unknown=Empty())
#     if timeframe.years is not None:
#         return sym_api.Timeframe(years=timeframe.years)
#     if timeframe.months is not None:
#         return sym_api.Timeframe(months=timeframe.months)
#     if timeframe.weeks is not None:
#         return sym_api.Timeframe(weeks=timeframe.weeks)
#     if timeframe.days is not None:
#         return sym_api.Timeframe(days=timeframe.days)
#     if timeframe.hours is not None:
#         return sym_api.Timeframe(hours=timeframe.hours)
#     if timeframe.minutes is not None:
#         return sym_api.Timeframe(minutes=timeframe.minutes)
#     if timeframe.seconds is not None:
#         return sym_api.Timeframe(seconds=timeframe.seconds)
#     if timeframe.ranges is not None:
#         return sym_api.Timeframe(ranges=timeframe.ranges)
#     if timeframe.ticks is not None:
#         return sym_api.Timeframe(ticks=timeframe.ticks)
#     raise ValueError(f"Cannot map qp.Timeframe to proto: {timeframe}")


# def _map_to_qp_ohlcv_bar(bar: ohlcv_api.OhlcvBar) -> qp.OhlcvBar:
#     open_time = bar.open_time.ToDatetime(tzinfo=timezone.utc)
#     close_time = bar.close_time.ToDatetime(tzinfo=timezone.utc)
#     return qp.OhlcvBar(
#         open_time=open_time,
#         close_time=close_time,
#         open=bar.open,
#         high=bar.high,
#         low=bar.low,
#         close=bar.close,
#         volume=bar.volume,
#     )
