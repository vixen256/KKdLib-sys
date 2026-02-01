#include <rdo_bc_encoder.h>

extern "C" {
uint8_t *
rdo_encode_rgba (uint8_t *rgba, uint32_t width, uint32_t height, DXGI_FORMAT format) {
	rdo_bc::rdo_bc_params rp;
	rp.m_rdo_lambda  = 0.2;
	rp.m_dxgi_format = format;

	utils::image_u8 img (width, height);
	memcpy (img.get_pixels ().data (), rgba, width * height * 4);

	rdo_bc::rdo_bc_encoder encoder;
	if (!encoder.init (img, rp)) return nullptr;
	if (!encoder.encode ()) return nullptr;

	uint32_t size = encoder.get_total_blocks_size_in_bytes ();
	uint8_t *out  = (uint8_t *)malloc (size);
	memcpy (out, encoder.get_blocks (), size);
	return out;
}

uint8_t *
rdo_encode_rg (uint8_t *rg, uint32_t width, uint32_t height, DXGI_FORMAT format) {
	rdo_bc::rdo_bc_params rp;
	rp.m_rdo_lambda  = 0.2;
	rp.m_dxgi_format = format;

	utils::image_u8 img (width, height);
	for (uint32_t i = 0; i < width * height; i++) {
		img.get_pixels ()[i].r = rg[i + 2 + 0];
		img.get_pixels ()[i].g = rg[i + 2 + 1];
	}

	rdo_bc::rdo_bc_encoder encoder;
	if (!encoder.init (img, rp)) return nullptr;
	if (!encoder.encode ()) return nullptr;

	uint32_t size = encoder.get_total_blocks_size_in_bytes ();
	uint8_t *out  = (uint8_t *)malloc (size);
	memcpy (out, encoder.get_blocks (), size);
	return out;
}

uint8_t *
rdo_decode_rgba (uint8_t *blocks, uint32_t width, uint32_t height, DXGI_FORMAT format) {
	rdo_bc::rdo_bc_params rp;
	rp.m_dxgi_format = format;

	utils::image_u8 img (width, height);

	rdo_bc::rdo_bc_encoder encoder;
	if (!encoder.init (img, rp)) return nullptr;
	memcpy ((void *)encoder.get_blocks (), blocks, encoder.get_total_blocks_size_in_bytes ());
	if (!encoder.unpack_blocks (img)) return nullptr;

	uint8_t *out = (uint8_t *)malloc (width * height * 4);
	memcpy (out, img.get_pixels ().data (), width * height * 4);
	return out;
}

uint8_t *
rdo_decode_rg (uint8_t *blocks, uint32_t width, uint32_t height, DXGI_FORMAT format) {
	rdo_bc::rdo_bc_params rp;
	rp.m_dxgi_format = format;

	utils::image_u8 img (width, height);

	rdo_bc::rdo_bc_encoder encoder;
	if (!encoder.init (img, rp)) return nullptr;
	memcpy ((void *)encoder.get_blocks (), blocks, encoder.get_total_blocks_size_in_bytes ());
	if (!encoder.unpack_blocks (img)) return nullptr;

	uint8_t *out = (uint8_t *)malloc (width * height * 2);
	for (uint32_t i = 0; i < width * height; i++) {
		out[i * 2 + 0] = img.get_pixels ()[i].r;
		out[i * 2 + 1] = img.get_pixels ()[i].g;
	}
	return out;
}

void
rdo_free (uint8_t *ptr) {
	free (ptr);
}
}
