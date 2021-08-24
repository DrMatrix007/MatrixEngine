using System;
using MatrixEngine.Renderers;
using MatrixEngine.System.Math;

namespace MatrixEngine.GameObjects.Components.LightComponents {
    public class LightBulbComponent : LightComponent {




        private float _intensity;

        private float _maxPower;


        public float intensity
        {
            get => _intensity;
            set => _intensity = Math.Clamp(value, 0.0f, 1.0f);
        }

        public float maxPower
        {
            get => _maxPower;
            set => _maxPower = Math.Clamp(value, 0.0f, 1.0f);
        }

        public float shadowToLightRatio { get; set; }

        public LightBulbComponent(float intensity, float maxPower, float shadowToLightRatio) {
            _intensity = intensity;
            _maxPower = maxPower;
            this.shadowToLightRatio = shadowToLightRatio;
        }


        internal override LightRenderer.LightType lightType { get; } = LightRenderer.LightType.Bulb;
    }
}