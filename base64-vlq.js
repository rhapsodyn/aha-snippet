const VLQ_BASE_SHIFT = 5
const VLQ_BASE = 1 << VLQ_BASE_SHIFT
const VLQ_BASE_MASK = VLQ_BASE - 1
const VLQ_CONTINUATION_BIT = VLQ_BASE
const BASE64_DIGITS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/'

function encode(value)
{
  // 末位补0
  if (value < 0)
    value = ((-value) << 1) | 1;
  else
    value <<= 1;

  result = "";
  do {
    // 取低5位
    digit = value & VLQ_BASE_MASK;
    // 右移
    value >>>= VLQ_BASE_SHIFT;
    if (value > 0)
      // 补首位标致位
      digit |= VLQ_CONTINUATION_BIT;
    // base64
    result += BASE64_DIGITS[digit];
  } while (value > 0);

  return result;
}

console.log(encode(65535))
