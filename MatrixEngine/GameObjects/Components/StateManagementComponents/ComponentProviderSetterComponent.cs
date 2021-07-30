using MatrixEngine.GameObjects.StateManagment;

namespace MatrixEngine.GameObjects.Components.StateManagementComponents {
    public class ComponentProviderSetterComponent<Comp> : Component where Comp : Component {
        private readonly Provider<Comp> provider;

        public ComponentProviderSetterComponent(Provider<Comp> provider) {
            this.provider = provider;
        }
        public override void Setup() {
            provider.data = GetComponent<Comp>();

        }

        public override void Start() {
        }

        public override void Update() {
            provider.data = GetComponent<Comp>();
        }
    }
}
