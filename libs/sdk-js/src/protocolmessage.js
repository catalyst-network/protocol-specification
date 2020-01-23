/**
 * @fileoverview
 * @enhanceable
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!

goog.provide('proto.Catalyst.Protocol.Wire.ProtocolMessage');

goog.require('jspb.Message');
goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('proto.Catalyst.Protocol.Cryptography.Signature');
goog.require('proto.Catalyst.Protocol.Peer.PeerId');


/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.Catalyst.Protocol.Wire.ProtocolMessage, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.Catalyst.Protocol.Wire.ProtocolMessage.displayName = 'proto.Catalyst.Protocol.Wire.ProtocolMessage';
}


if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.toObject = function(opt_includeInstance) {
  return proto.Catalyst.Protocol.Wire.ProtocolMessage.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.Catalyst.Protocol.Wire.ProtocolMessage} msg The msg instance to transform.
 * @return {!Object}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.toObject = function(includeInstance, msg) {
  var f, obj = {
    peerId: (f = msg.getPeerId()) && proto.Catalyst.Protocol.Peer.PeerId.toObject(includeInstance, f),
    correlationId: msg.getCorrelationId_asB64(),
    typeUrl: jspb.Message.getFieldWithDefault(msg, 3, ""),
    value: msg.getValue_asB64(),
    signature: (f = msg.getSignature()) && proto.Catalyst.Protocol.Cryptography.Signature.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.Catalyst.Protocol.Wire.ProtocolMessage}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.Catalyst.Protocol.Wire.ProtocolMessage;
  return proto.Catalyst.Protocol.Wire.ProtocolMessage.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.Catalyst.Protocol.Wire.ProtocolMessage} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.Catalyst.Protocol.Wire.ProtocolMessage}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.Catalyst.Protocol.Peer.PeerId;
      reader.readMessage(value,proto.Catalyst.Protocol.Peer.PeerId.deserializeBinaryFromReader);
      msg.setPeerId(value);
      break;
    case 2:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setCorrelationId(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setTypeUrl(value);
      break;
    case 4:
      var value = /** @type {!Uint8Array} */ (reader.readBytes());
      msg.setValue(value);
      break;
    case 5:
      var value = new proto.Catalyst.Protocol.Cryptography.Signature;
      reader.readMessage(value,proto.Catalyst.Protocol.Cryptography.Signature.deserializeBinaryFromReader);
      msg.setSignature(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.Catalyst.Protocol.Wire.ProtocolMessage.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.Catalyst.Protocol.Wire.ProtocolMessage} message
 * @param {!jspb.BinaryWriter} writer
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getPeerId();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.Catalyst.Protocol.Peer.PeerId.serializeBinaryToWriter
    );
  }
  f = message.getCorrelationId_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      2,
      f
    );
  }
  f = message.getTypeUrl();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getValue_asU8();
  if (f.length > 0) {
    writer.writeBytes(
      4,
      f
    );
  }
  f = message.getSignature();
  if (f != null) {
    writer.writeMessage(
      5,
      f,
      proto.Catalyst.Protocol.Cryptography.Signature.serializeBinaryToWriter
    );
  }
};


/**
 * optional Catalyst.Protocol.Peer.PeerId peer_id = 1;
 * @return {?proto.Catalyst.Protocol.Peer.PeerId}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.getPeerId = function() {
  return /** @type{?proto.Catalyst.Protocol.Peer.PeerId} */ (
    jspb.Message.getWrapperField(this, proto.Catalyst.Protocol.Peer.PeerId, 1));
};


/** @param {?proto.Catalyst.Protocol.Peer.PeerId|undefined} value */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.setPeerId = function(value) {
  jspb.Message.setWrapperField(this, 1, value);
};


proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.clearPeerId = function() {
  this.setPeerId(undefined);
};


/**
 * Returns whether this field is set.
 * @return {!boolean}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.hasPeerId = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional bytes correlation_id = 2;
 * @return {string}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.getCorrelationId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * optional bytes correlation_id = 2;
 * This is a type-conversion wrapper around `getCorrelationId()`
 * @return {string}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.getCorrelationId_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getCorrelationId()));
};


/**
 * optional bytes correlation_id = 2;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getCorrelationId()`
 * @return {!Uint8Array}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.getCorrelationId_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getCorrelationId()));
};


/** @param {!(string|Uint8Array)} value */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.setCorrelationId = function(value) {
  jspb.Message.setField(this, 2, value);
};


/**
 * optional string type_url = 3;
 * @return {string}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.getTypeUrl = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/** @param {string} value */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.setTypeUrl = function(value) {
  jspb.Message.setField(this, 3, value);
};


/**
 * optional bytes value = 4;
 * @return {string}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.getValue = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * optional bytes value = 4;
 * This is a type-conversion wrapper around `getValue()`
 * @return {string}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.getValue_asB64 = function() {
  return /** @type {string} */ (jspb.Message.bytesAsB64(
      this.getValue()));
};


/**
 * optional bytes value = 4;
 * Note that Uint8Array is not supported on all browsers.
 * @see http://caniuse.com/Uint8Array
 * This is a type-conversion wrapper around `getValue()`
 * @return {!Uint8Array}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.getValue_asU8 = function() {
  return /** @type {!Uint8Array} */ (jspb.Message.bytesAsU8(
      this.getValue()));
};


/** @param {!(string|Uint8Array)} value */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.setValue = function(value) {
  jspb.Message.setField(this, 4, value);
};


/**
 * optional Catalyst.Protocol.Cryptography.Signature signature = 5;
 * @return {?proto.Catalyst.Protocol.Cryptography.Signature}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.getSignature = function() {
  return /** @type{?proto.Catalyst.Protocol.Cryptography.Signature} */ (
    jspb.Message.getWrapperField(this, proto.Catalyst.Protocol.Cryptography.Signature, 5));
};


/** @param {?proto.Catalyst.Protocol.Cryptography.Signature|undefined} value */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.setSignature = function(value) {
  jspb.Message.setWrapperField(this, 5, value);
};


proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.clearSignature = function() {
  this.setSignature(undefined);
};


/**
 * Returns whether this field is set.
 * @return {!boolean}
 */
proto.Catalyst.Protocol.Wire.ProtocolMessage.prototype.hasSignature = function() {
  return jspb.Message.getField(this, 5) != null;
};


