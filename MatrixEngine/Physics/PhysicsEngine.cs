using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.GameObjects.Components.TilemapComponents;
using MatrixEngine.System;
using SFML.System;
using System;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine.Physics {
    public class PhysicsEngine {

        public const float Threshold = 0.010f;



        private List<RigidBodyComponent> dynamicRigidBodies;
        private List<ColliderComponent> colliders;


        public System.App app
        {
            get;
            private set;
        }

        public PhysicsEngine(System.App app) {
            this.app = app;
            dynamicRigidBodies = new List<RigidBodyComponent>();
            colliders = new List<ColliderComponent>();
        }

        public void AddRigidbodyToFrame(RigidBodyComponent rigidBodyComponent) {

            dynamicRigidBodies.Add(rigidBodyComponent);


        }
        public void AddColliderToFrame(ColliderComponent rect) {
            colliders.Add(rect);
        }

        public void Update() {


            foreach (var item in dynamicRigidBodies) {
                if (!item.isStatic) {

                    item.velocity += (item.gravity * app.deltaTime).Log();
                    var fric = (item.velocity.Length() < 1 ? item.velocity : item.velocity.Normalize()).Multiply(item.velocityDrag) * (-1) / 1.0f * app.deltaTime * 100;

                    item.velocity += fric;

                    item.position += (item.velocity * app.deltaTime);


                }
            }





            var static_list = colliders.ToArray();
            var non_static_list = dynamicRigidBodies.ToArray();


            foreach (var @static in static_list) {

                if (@static.colliderType == ColliderComponent.ColliderType.None) {
                    continue;
                }

                foreach (var nonstatic in non_static_list) {

                    if (nonstatic.colliderComponent.colliderType == ColliderComponent.ColliderType.None) {
                        continue;
                    }

                    if (nonstatic.colliderComponent.colliderType == ColliderComponent.ColliderType.Rect) {

                        if (@static.colliderType == ColliderComponent.ColliderType.Rect) {
                            HandleRectToRect(nonstatic, @static.rect);
                        }
                        if (@static.colliderType == ColliderComponent.ColliderType.Tilemap) {
                            HandleRectToTilemap(nonstatic, @static);
                        }


                    }

                }
            }






            dynamicRigidBodies.Clear();
            colliders.Clear();

        }

        private void HandleRectToTilemap(RigidBodyComponent nonstatic, ColliderComponent @static) {
            var tilemap = @static.GetComponent<TilemapComponent>();
            if (tilemap == null) {
                return;
            }

            var nonstatic_rect = nonstatic.colliderComponent.rect;
            var tile_scale = tilemap.transform.scale;

            var list_rects = new List<Rect>();

            var pos = new Vector2f(0, 0);

            for (float x = -tile_scale.X * 2; x < nonstatic_rect.width + tile_scale.X * 2; x += tile_scale.X) {
                for (float y = -tile_scale.Y * 2; y < nonstatic_rect.height + tile_scale.Y * 2; y += tile_scale.Y) {
                    pos = new Vector2f(x, y) + tilemap.position;
                    if (tilemap.GetTileFromWorldPos(pos + nonstatic.position) != null) {
                        var r = new Rect((pos + (Vector2f)(Vector2i)nonstatic.position.Round(10)), tile_scale);
                        //if (x == (nonstatic.velocity.X>0?0: tile_scale.X) || y == (nonstatic.velocity.Y > 0 ? tile_scale.Y : 0)) {
                        list_rects.Insert(0, r);
                        //} else {
                        list_rects.Add(r);

                        //}


                    }
                }
            }
            foreach (var item in list_rects.OrderBy((o)=> {
            
                   return nonstatic.transform.fullRect.center.Distance(o.center); 

            })) {
                //item.position.Log();
                HandleRectToRect(nonstatic, item);

            }


        }





        void HandleRectToRect(RigidBodyComponent nonstatic, Rect @static) {
            var result = nonstatic.gameObject.transform.fullRect.GetCollidingFixFromRect(@static);


            if (!result.isCollide) {
                return;
            }
            var isX = result.fixValue.X.Abs() < result.fixValue.Y.Abs();

            if (isX) {

                var isleft = nonstatic.transform.fullRect.cX < @static.cX;
                if (isleft) {
                    nonstatic.position = new Vector2f(@static.position.X - nonstatic.transform.fullRect.width, nonstatic.position.Y);
                } else {
                    nonstatic.position = new Vector2f(@static.max.X, nonstatic.position.Y);

                }


                nonstatic.velocity = new Vector2f(0, nonstatic.velocity.Y);



            } else {
                var isup = nonstatic.transform.fullRect.cY < @static.cY;

                if (isup) {

                    nonstatic.position = new Vector2f(nonstatic.position.X,@static.position.Y-nonstatic.transform.fullRect.height);

                } else {
                    nonstatic.position = new Vector2f(nonstatic.position.X, @static.max.Y);

                }
                nonstatic.velocity = new Vector2f(nonstatic.velocity.X, 0);


            }

        }
    }

}

