using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.GameObjects.Components {
    [AttributeUsage(AttributeTargets.Class,AllowMultiple =true,Inherited =true)]
    public class RequireComponent : Attribute {
        
        public Type type;

        public RequireComponent(Type componentType) {
            type = componentType;
            var allowed = false;
            if (!componentType.IsSubclassOf(typeof(Component))) {
                throw new ArgumentException($"Type {type.FullName} is not subclass of {typeof(Component).FullName} as it should be.");
            }
            foreach (var item in type.GetConstructors()) {
                if (item.GetParameters().Length == 0) {
                    allowed = true;
                }
            }
            if (!allowed) {
                throw new ArgumentException($"Type {type.FullName} don't have defualt Constructor, so it can't be instantiated automaticly!");
            }
        }
    }

}
